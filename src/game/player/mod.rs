use bevy::prelude::*;
use bevy::transform::TransformSystem::TransformPropagate;
use crate::AppState;
use crate::game::player::systems::{life_time_system, move_camera_system, move_player, paint_target, shoot, spawn_player};
use crate::general::systems::update_cursor_system;

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    // paint_target.before(update_cursor_system),
                    move_player,
                    paint_target,
                    move_camera_system,
                    shoot,
                    life_time_system,

                    // bullet_collisions_system
                ).in_set(OnUpdate(AppState::Game)).chain()
            )
            // .add_system(move_camera_system.in_base_set(CoreSet::FixedUpdate))
            // .add_system(paint_target.in_base_set(CoreSet::PreUpdate))
        ;
    }
}