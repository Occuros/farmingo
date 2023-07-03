use bevy::prelude::*;
use bevy::transform::TransformSystem::TransformPropagate;
use crate::AppState;
use crate::game::player::systems::{bullet_collisions_system, life_time_system, move_camera_system, move_player, paint_target, shoot, shoot_something, spawn_player, Target};

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Target {position: Vec3::ZERO})
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    move_player,
                    move_camera_system.after(move_player),
                    // shoot_something.before(move_player),
                    // paint_target.after(shoot_something),
                    shoot,
                    life_time_system,
                    // bullet_collisions_system
                ).in_set(OnUpdate(AppState::Game))
            )
            // .add_system(shoot_something.in_base_set(CoreSet::PostUpdate))
            // .add_system(paint_target.in_base_set(CoreSet::PostUpdate))
        ;
    }
}