use bevy::prelude::*;
use smooth_bevy_cameras::{LookAngles, LookTransform};
use crate::game::player::components::Player;

pub const PLAYER_SPEED: f32 = 1.0;


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
    Player{}
    ));
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
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
    mut cameras: Query<&mut Transform, (With<Camera>, Without<Player>)>,
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