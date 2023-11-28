use bevy::prelude::*;

use crate::AppState;

use self::{
    components::{PathFoundEvent, PathRequestEvent},
    systems::{find_path_system, input_for_testing_system, debug_path_finding},
};

mod components;
mod systems;

pub struct ExperimentsPlugin;

impl Plugin for ExperimentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PathRequestEvent>()
            .add_event::<PathFoundEvent>()
            .add_systems(
                Update,
                (
                    find_path_system, 
                    input_for_testing_system,
                    debug_path_finding,
                ).run_if(in_state(AppState::Game)),
            );
    }
}
