use bevy::app::App;
use bevy::prelude::*;
use crate::general::components::GameCursor;
use crate::general::systems::update_cursor_system;

pub mod components;
pub mod systems;

pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GameCursor::default())
            .add_system(update_cursor_system)
            // .add_system(update_cursor_system.in_base_set(CoreSet::FixedUpdate))

        ;
    }
}