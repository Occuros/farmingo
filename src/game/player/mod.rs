use bevy::prelude::*;
use crate::AppState;
use crate::game::player::systems::{bullet_collisions_system, life_time_system, move_camera_system, move_player, paint_target, shoot, spawn_player};

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    move_player,
                    move_camera_system.after(move_player),
                    shoot,
                    life_time_system,
                    bullet_collisions_system,
                ).in_set(OnUpdate(AppState::Game))
            )
            .add_system(paint_target.in_base_set(CoreSet::PostUpdate))
        ;
    }
}