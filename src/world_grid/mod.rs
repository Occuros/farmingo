use bevy::prelude::*;
use crate::world_grid::components::WorldGrid;
use crate::world_grid::systems::{debug_world_system, draw_grid};

pub mod components;
mod systems;

pub struct WorldGridPlugin;

impl Plugin for WorldGridPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WorldGrid::new(20, 10, 1.0))
            .add_startup_system(debug_world_system)
            .add_system(draw_grid)
        ;

    }
}