use std::cmp::max;
use std::f32::consts::TAU;
use bevy::input::mouse::MouseButtonInput;
use bevy::math::quat;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_mod_picking::backend::HitData;
use bevy_mod_picking::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::prelude::Cuboid;
use bevy_vector_shapes::prelude::*;
// use smooth_bevy_cameras::{LookAngles, LookTransform};
use crate::game::player::components::{BulletBundle, LifeTime, Player};
use crate::{DoSomethingComplex, MainCamera};

pub const PLAYER_SPEED: f32 = 2.0;

#[derive(Resource)]
pub struct Target {
    pub position: Vec3,
}


pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.50 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.25, 0.0),
        ..default()
    },
                    Player {},
                    Collider::cuboid(0.5, 0.5, 0.5)
    ));
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;

    if let Ok(mut transform) = player_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 0.0, -1.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, 0.0, 1.0);
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}


// pub fn move_camera_system(
//     mut cameras: Query<&mut LookTransform>,
//     player_query: Query<&Transform, With<Player>>,
// ) {
//     if let Ok(player_transform) = player_query.get_single() {
//         for mut c in cameras.iter_mut() {
//             // let mut angles = LookAngles::from_vector(c.look_direction().unwrap());
//             // angles.add_pitch(0.00000001);
//             // angles.add_yaw(delta.x);
//             c.target = player_transform.translation;
//             c.eye = player_transform.translation - player_transform.forward() * 3.0 + player_transform.up() * 12.0;
//             // c.eye = c.target + 1.0 * c.radius() * angles.unit_vector();
//         }
//     }
// }

pub fn move_camera_system(
    mut cameras: Query<&mut Transform, (With<Camera>, With<MainCamera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut c in cameras.iter_mut() {
            let look_target = player_transform.translation - player_transform.forward() * 3.0 + player_transform.up() * 12.0;
            c.translation = look_target;
            c.look_at(player_transform.translation, Vec3::Y);
        }
    }
}

pub fn shoot_something(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<Camera>, With<MainCamera>, Without<Player>)>,
    rapier: Res<RapierContext>,
    mut target: ResMut<Target>,
    time: Res<Time>,
) {
    let window = window_query.get_single().unwrap();
    let (camera, camera_transform) = camera_query.get_single().unwrap();

    if let Some(cursor_position) = window.cursor_position() {
        let ray: Option<Ray> = camera.viewport_to_world(camera_transform, cursor_position);
        let filter: QueryFilter = QueryFilter::default();
        if let Some(ray) = ray {
            let hit = rapier.cast_ray(ray.origin, ray.direction, f32::MAX, true, filter);

            if let Some((_, toi)) = hit {
                let position = ray.origin + ray.direction * toi;
               // target.position = target.position.lerp(position, time.delta_seconds() * 10.0);
                target.position = position;
            }
        }
    }
}

pub fn paint_target(
    target: Res<Target>,
    mut painter: ShapePainter,
    time: Res<Time>,
) {
    painter.transform.translation = target.position + Vec3::Y * 0.01;
    painter.transform.rotation = Quat::from_rotation_x(TAU * 0.25);
    painter.hollow = false;
    painter.color = Color::ORANGE;
    painter.circle(0.3);
}

pub fn shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    input: Res<Input<MouseButton>>,
    target: Res<Target>,
    player_query: Query<&Transform, (With<Player>)>,

) {
    let player_transform = player_query.single();
    let target_position = Vec3::new(target.position.x, player_transform.translation.y, target.position.z);
    let result = player_transform.looking_at(target_position, Vec3::Y);
    if input.just_pressed(MouseButton::Left) {
       commands.spawn(BulletBundle::new(player_transform.translation, result.rotation, meshes, materials));
    }
}

pub fn life_time_system(
    mut commands: Commands,
    time: Res<Time>,
    mut life_time_query: Query<(Entity, &mut LifeTime)>,
    mut test_query: Query<(&mut Transform, &Player)>,
) {
    for (e, mut life_time) in life_time_query.iter_mut() {
        life_time.time_left = (life_time.time_left - time.delta_seconds()).max(0.0);
        if life_time.time_left <= 0.0 {
            commands.entity(e).despawn()
        }
    }
}

