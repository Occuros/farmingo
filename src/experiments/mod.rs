
use bevy::prelude::*;



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