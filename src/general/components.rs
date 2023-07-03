use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera {}

#[derive(Resource, Default)]
pub struct GameCursor {
    pub ui_position: Option<Vec2>,
    pub world_position: Option<Vec3>,
}
