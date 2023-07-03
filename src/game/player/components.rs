use bevy::prelude::*;
use bevy_rapier3d::dynamics::Velocity;
use bevy_rapier3d::geometry::{ActiveCollisionTypes, Collider};
use bevy_rapier3d::prelude::{ActiveEvents, RigidBody};

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Bullet{}

#[derive(Component)]
pub struct LifeTime{
   pub time_left: f32,
}

#[derive(Bundle)]
pub struct BulletBundle {
    pbr_bundle: PbrBundle,
    bullet: Bullet,
    collider: Collider,
    rigid_body: RigidBody,
    velocity: Velocity,
    life_time: LifeTime,
    active_collision: ActiveEvents
}

impl BulletBundle {
    pub fn new(
        position: Vec3,
           rotation: Quat,
           mut meshes: ResMut<Assets<Mesh>>,
           mut materials: ResMut<Assets<StandardMaterial>>,) -> BulletBundle {
        let size = 0.1;

        let shape = shape::Icosphere {radius: size, subdivisions: 12};
        let transform = Transform::from_translation(position).with_rotation(rotation);
        Self {
            pbr_bundle: PbrBundle {
                transform,
                mesh: meshes.add(Mesh::try_from(shape).unwrap()),
                material: materials.add(Color::PURPLE.into()),
                ..default()
            },
            bullet: Bullet{},
            rigid_body: RigidBody::KinematicVelocityBased,
            collider: Collider::ball(size),
            velocity: Velocity {linvel: transform.forward() * 5.0, angvel: Vec3::ZERO},
            life_time: LifeTime{time_left: 5.0},
            active_collision: ActiveEvents::all()
        }
    }
}