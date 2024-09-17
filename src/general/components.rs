use bevy::prelude::*;
use avian3d::prelude::*;

#[derive(Component)]
pub struct MainCamera {}

#[derive(Resource, Default)]
pub struct GameCursor {
    pub ui_position: Option<Vec2>,
    pub world_position: Option<Vec3>,
}

#[derive(Component, Default)]
pub struct StoredCollision {
    pub collisions: Vec<Collision>,
}
