use std::f32::consts::TAU;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_vector_shapes::prelude::*;
use crate::game::player::components::{Bullet, BulletBundle, LifeTime, Player};
use crate::general::components::{GameCursor, MainCamera};

pub const PLAYER_SPEED: f32 = 2.0;


pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.50 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.25, 0.0),
            ..default()
        },
        Player { local_aim_target: Vec3::ZERO },
        Collider::cuboid(0.25, 0.25, 0.25),
        Name::new("Player")
    ));
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    game_cursor: Res<GameCursor>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;

    if let Ok((mut transform, mut player)) = player_query.get_single_mut() {
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
        player.local_aim_target = transform.transform_point(game_cursor.world_position.unwrap_or_default());
    }
}


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


pub fn paint_target(
    game_cursor: Res<GameCursor>,
    mut painter: ShapePainter,
) {
    if game_cursor.world_position.is_none() { return; };
    let position = game_cursor.world_position.unwrap();
    painter.set_translation(position);
    painter.transform.translation += Vec3::Y * 0.01;
    painter.transform.rotation = Quat::from_rotation_x(TAU * 0.25);
    painter.hollow = false;
    painter.color = Color::ORANGE;
    painter.circle(0.3);

}

pub fn shoot(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    input: Res<Input<MouseButton>>,
    game_cursor: Res<GameCursor>,
    player_query: Query<&Transform, With<Player>>,
) {
    if game_cursor.world_position.is_none() { return; };
    let target = game_cursor.world_position.unwrap();
    let player_transform = player_query.single();
    let target_position = Vec3::new(target.x, player_transform.translation.y, target.z);
    let result = player_transform.looking_at(target_position, Vec3::Y);
    if input.just_pressed(MouseButton::Left) {
        commands.spawn(BulletBundle::new(player_transform.translation, result.rotation, meshes, materials));
    }
}

pub fn life_time_system(
    mut commands: Commands,
    time: Res<Time>,
    mut life_time_query: Query<(Entity, &mut LifeTime)>,
) {
    for (e, mut life_time) in life_time_query.iter_mut() {
        life_time.time_left = (life_time.time_left - time.delta_seconds()).max(0.0);
        if life_time.time_left <= 0.0 {
            commands.entity(e).despawn()
        }
    }
}

pub fn bullet_collisions_system(
    mut commands: Commands,
    bullet_query: Query<&Bullet>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if let Ok(_) = bullet_query.get(*e1) {
                    commands.entity(*e1).despawn();
                }

                if let Ok(_) = bullet_query.get(*e2) {
                    commands.entity(*e2).despawn();
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
    //
    // for contact_force_event in contact_force_events.iter() {
    //     println!("Received contact force event: {:?}", contact_force_event);
    // }
}
