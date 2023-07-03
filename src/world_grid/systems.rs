use std::f32::consts::TAU;
use bevy::math::{quat, Quat, Vec2};
use bevy::prelude::*;
use bevy_mod_picking::pointer;
use bevy_rapier3d::pipeline::QueryFilter;
use bevy_rapier3d::prelude::*;
use bevy_vector_shapes::prelude::*;
use crate::world_grid::components::{GridCursor, WorldGrid};

pub fn debug_world_system(
    world_grid: Res<WorldGrid>,
    mut painter: ShapePainter,
) {
    println!("world gird size: w:{} h:{}", world_grid.width, world_grid.height);
    // for cell in &world_grid {
    //     println!("cells: x:{}, y:{}", cell.position.x, cell.position.y);
    // }
}


pub fn draw_grid(
    mut commands: Commands,
    mut painter: ShapePainter, world_grid: Res<WorldGrid>,
    asset_server: Res<AssetServer>,
) {


    // for (e, id, p_location) in pointers.iter() {
    //     if let Some(location) = &p_location.location {
    //         let text = format!("{id:?}");
    //
    //         commands.entity(e).insert(TextBundle {
    //             text: Text::from_section(
    //                 text,
    //                 TextStyle {
    //                     font: asset_server.load("fonts/FiraMono-Medium.ttf"),
    //                     font_size: 12.0,
    //                     color: Color::WHITE,
    //                 },
    //             ),
    //             style: Style {
    //                 position_type: PositionType::Absolute,
    //                 position: UiRect {
    //                     left: Val::Px(location.position.x + 5.0),
    //                     bottom: Val::Px(location.position.y + 5.0),
    //                     ..default()
    //                 },
    //                 ..default()
    //             },
    //             ..default()
    //         });
    //     }
    //
    //
    // }
    let rotation = Quat::from_rotation_x(TAU * 0.25);
    painter.set_rotation(rotation);
    painter.hollow = true;
    painter.thickness = 0.01;
    painter.color = Color::WHITE;
    for cell in &world_grid {
        let mut position = world_grid.grid_to_world(&cell.position);
        position.y += 0.001;
        painter.transform.translation = position;
        // painter.circle(world_grid.grid_size * 0.5);
        painter.rect(Vec2::splat(world_grid.grid_size));
    }
}
