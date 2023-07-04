use bevy::prelude::*;
use crate::world_grid::components::WorldGrid;
use crate::world_grid::systems::{debug_spawn_grid_positions, debug_world_system, draw_grid, gird_test_system, update_grid_positions};

pub mod components;
mod systems;

pub struct WorldGridPlugin;

impl Plugin for WorldGridPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WorldGrid::new(20, 10, 1.0))
            .add_startup_system(debug_world_system)
            .add_startup_system(debug_spawn_grid_positions)
            .add_startup_system(gird_test_system.before(debug_spawn_grid_positions))
            .add_system(update_grid_positions)
            .add_system(draw_grid)
        ;

    }
}