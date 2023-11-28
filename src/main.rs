mod experiments;
mod game;
mod general;
mod world_grid;

use crate::game::player::components::Player;
use crate::game::GamePlugin;
use crate::general::components::MainCamera;
use crate::general::GeneralPlugin;
use crate::world_grid::WorldGridPlugin;
use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_mod_billboard::prelude::BillboardPlugin;
use bevy_mod_picking::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use bevy_turborand::prelude::*;
use bevy_vector_shapes::ShapePlugin;
use bevy_xpbd_3d::prelude::*;
use experiments::ExperimentsPlugin;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    MainMenu,
    #[default]
    Game,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(
            DefaultPickingPlugins
                .build()
                .disable::<DebugPickingPlugin>(),
        )
        .add_plugins(BillboardPlugin)
        .add_plugins(EditorPlugin::default())
        .add_plugins(ShapePlugin::default())
        .add_plugins(RngPlugin::default())
        .add_plugins(GeneralPlugin)
        .add_plugins(WorldGridPlugin)
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_state::<AppState>()
        .add_plugins(GamePlugin)
        .add_plugins(ExperimentsPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, move_light_system)
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
        // RaycastPickTarget::default(), // Marker for the `bevy_picking_raycast` backend
        // OnPointer::<Over>::send_event::<DoSomethingComplex>(),
        Collider::cuboid(25.0, 0.01, 25.0),
        RigidBody::Static,
        Name::new("Floor"),
    ));

    for _i in 0..30 {
        let size = 0.5;
        let max_position = 20.0;
        let position = Vec3::new(
            rng.f32_normalized() * max_position,
            size * 0.5 + 10.0,
            rng.f32_normalized() * max_position,
        );

        // cube
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.50 })),
                material: materials.add(Color::rgb(0.8, rng.f32(), 0.6).into()),
                transform: Transform::from_translation(position),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::cuboid(size * 0.5, size * 0.5, size * 0.5),
            Friction::new(0.9)
                .with_dynamic_coefficient(0.9)
                .with_combine_rule(CoefficientCombine::Max),
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

    let _eye = Vec3::new(-0.2, 2.5, 5.0);
    let _target = Vec3::default();

    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(MainCamera {})
        .insert(
            PickableBundle::default(), // Enable picking using this camera
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
) {
    let offset = Vec3::new(0.0, 8.0, 2.0);
    if let Ok(player_transform) = player_query.get_single() {
        for mut light_transform in light_query.iter_mut() {
            light_transform.translation = player_transform.translation + offset;
        }
    }
}
