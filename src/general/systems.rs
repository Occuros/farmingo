
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::general::components::{GameCursor, MainCamera};

pub fn update_cursor_system(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), (With<Camera>, With<MainCamera>)>,
    rapier: Res<RapierContext>,
    mut game_cursor: ResMut<GameCursor>,
) {
    let window = window_query.get_single().unwrap();
    let (camera, camera_transform) = camera_query.get_single().unwrap();
    game_cursor.ui_position = window.cursor_position();
    if let Some(cursor_position) = window.cursor_position() {
        let ray: Option<Ray> = camera.viewport_to_world(camera_transform, cursor_position);
        let filter: QueryFilter = QueryFilter::default();
        if let Some(ray) = ray {
            let hit = rapier.cast_ray(ray.origin, ray.direction, f32::MAX, true, filter);

            if let Some((_, toi)) = hit {
                let position = ray.origin + ray.direction * toi;
                game_cursor.world_position = Some(position);
            }


        }
    }
}
