use bevy::prelude::*;

#[derive(Default, Component)]
pub struct TimeKeeper {
    pub elapsed_time: f32,
    pub delta_time: f32,
}
