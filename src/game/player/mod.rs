use bevy::prelude::*;
use crate::AppState;
use crate::game::player::systems::{move_camera_system, move_player, spawn_player};

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
                    move_camera_system,
                ).in_set(OnUpdate(AppState::Game))
            )
        ;
    }
}