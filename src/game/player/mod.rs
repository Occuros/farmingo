use bevy::prelude::*;
use crate::AppState;
use crate::game::player::systems::*;

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Game),spawn_player)
            .add_systems(
                Update,(
                    move_player,
                    move_camera_system.after(move_player),
                    shoot,
                    life_time_system,
                    bullet_collisions_system,
                    increse_cell_score_on_enter,
                    increase_cell_score_on_click,
                ).run_if(in_state(AppState::Game))
            )
            // .add_system(paint_target.in_base_set(CoreSet::PostUpdate))
        ;
    }
}