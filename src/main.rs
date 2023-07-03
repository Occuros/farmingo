mod game;
mod experiments;

use std::time::Duration;
use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_mod_picking::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng, RngPlugin};
use bevy_vector_shapes::ShapePlugin;
use bevy_xpbd_3d::prelude::*;
// use bevy_rapier3d::prelude::*;
use smooth_bevy_cameras::{LookAngles, LookTransform, LookTransformBundle, LookTransformPlugin, Smoother};
use crate::experiments::ExperimentsPlugin;
use crate::game::GamePlugin;
use crate::game::player::components::Player;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    MainMenu,
    #[default]
    Game,
    GameOver,
}

#[derive(Component)]
pub struct MainCamera {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(PhysicsPlugins)
        // .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(EditorPlugin::default())
        .add_plugin(ShapePlugin::default())
        .add_plugin(LookTransformPlugin)
        .add_plugin(RngPlugin::default())
        .add_plugin(ExperimentsPlugin)
        .add_event::<DoSomethingComplex>()
        .add_state::<AppState>()
        .add_plugin(GamePlugin)
        .add_startup_system(setup)
        .add_system(move_light_system)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: ResMut<GlobalRng>,
) {
    // plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(50.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        // PickableBundle::default(),
        // RaycastPickTarget::default(),    // Marker for the `bevy_picking_raycast` backend
        // OnPointer::<Over>::send_event::<DoSomethingComplex>(),
        RigidBody::Static,
        Friction::new(1.0),
        Collider::cuboid(25.0, 0.01, 25.0),
        Name::new("Floor"),
    ));

    for i in 0..30 {
        let size = 0.5;
        let max_position = 20.0;
        let position = Vec3::new(rng.f32_normalized() * max_position, size * 0.5 + 10.0, rng.f32_normalized() * max_position);

        // cube
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.50 })),
                material: materials.add(Color::rgb(0.8, rng.f32(), 0.6).into()),
                transform: Transform::from_translation(position),
                ..default()
            },
            RigidBody::Dynamic,
            Friction::new(1.0).with_combine_rule(CoefficientCombine::Max),
            Position(position),
            Collider::cuboid(size, size, size),
            Name::new("cube"),
        ));
        // .insert(RigidBody::Dynamic)
        // .insert(Position(position))
        // .insert(Collider::cuboid(size, size, size))
        // .insert(Collider::cuboid(size * 0.5, size * 0.5, size * 0.5))
        // .insert(RigidBody::Dynamic)
    }


    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0 * 2.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // directional 'sun' light
    // commands.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         shadows_enabled: true,
    //         illuminance: 800.0,
    //         ..default()
    //     },
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 2.0, 0.0),
    //         rotation: Quat::from_rotation_x(-PI / 4.),
    //         ..default()
    //     },
    //     ..default()
    // });

    let eye = Vec3::new(-0.2, 2.5, 5.0);
    let target = Vec3::default();


    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
        .insert(MainCamera {})
        .insert(RaycastPickCamera::default(),   // Enable picking using this camera
        );


    // camera
    // commands.spawn(Camera3dBundle {
    //     ..default()
    // }).insert(LookTransformBundle {
    //     transform: LookTransform::new(eye, target, Vec3::Y),
    //     smoother: Smoother::new(0.9),
    // });
}

fn move_light_system(
    mut light_query: Query<&mut Transform, (With<PointLight>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<PointLight>)>,
)
{
    let offset = Vec3::new(0.0, 8.0, 2.0);
    if let Ok(player_transform) = player_query.get_single() {
        for mut light_transform in light_query.iter_mut() {
            light_transform.translation = player_transform.translation + offset;
        }
    }
}


pub struct DoSomethingComplex(Entity, f32, Option<Vec3>);

impl From<ListenedEvent<Over>> for DoSomethingComplex {
    fn from(event: ListenedEvent<Over>) -> Self {
        DoSomethingComplex(event.target, event.hit.depth, event.hit.position)
    }
}