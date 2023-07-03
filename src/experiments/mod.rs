use std::time::Duration;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use crate::experiments::systems::limited_rate_system;

mod systems;
mod components;


pub struct ExperimentsPlugin;

impl Plugin for ExperimentsPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_system(limited_rate_system.run_if(on_timer(Duration::from_secs_f32(2.0))))
        ;
    }
}