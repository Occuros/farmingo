use bevy::prelude::*;
use bevy::time::Time;
use crate::experiments::components::TimeKeeper;
#[allow(dead_code)]
pub fn limited_rate_system(time: Res<Time>, mut time_keeper: Local<TimeKeeper>) {
    time_keeper.delta_time = time.elapsed_seconds() - time_keeper.elapsed_time;
    time_keeper.elapsed_time = time.elapsed_seconds();
    println!("Time elapsed: {}", time_keeper.delta_time)
}