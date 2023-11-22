use bevy::math::{Quat, Vec2};
use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;
use std::f32::consts::TAU;

use crate::general::components::GameCursor;
use crate::world_grid::components::{Cell, GridPosition, WorldGrid};
use bevy_vector_shapes::prelude::*;

pub fn debug_world_system(world_grid: Res<WorldGrid>, _painter: ShapePainter) {
    println!(
        "world gird size: w:{} h:{}",
        world_grid.width, world_grid.height
    );
    // for cell in &world_grid {
    //     println!("cells: x:{}, y:{}", cell.position.x, cell.position.y);
    // }
}

pub fn debug_spawn_grid_positions(
    mut commands: Commands,
    world_grid: Res<WorldGrid>,
    asset_server: Res<AssetServer>,
) {
    for cell in world_grid.into_iter() {
        let mut position = world_grid.grid_to_world(&cell.position);
        position.y += 0.03;
        let rotation = Quat::from_rotation_x(TAU * 0.25);

        commands.spawn((
            BillboardTextBundle {
                transform: bevy::prelude::Transform::from_translation(position)
                    .with_rotation(rotation)
                    .with_scale(Vec3::splat(0.01)),
                text: Text::from_sections([TextSection {
                    value: format!("{}", cell.value),
                    style: TextStyle {
                        font_size: 30.0,
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        color: Color::WHITE,
                    },
                }])
                .with_alignment(TextAlignment::Center),
                ..default()
            },
            GridPosition {
                x: cell.position.x,
                y: cell.position.y,
            },
            Cell::default(),
        ));
    }
}

pub fn update_grid_positions(
    world_grid: Res<WorldGrid>,
    mut grid_query: Query<(&mut Text, &mut Cell, &GridPosition)>,
) {
    for (mut text, mut cell, grid_position) in &mut grid_query {
        let updated_cell = &world_grid.cells[grid_position];
        if updated_cell.value != cell.value {
            // println!("we update value to {}", updated_cell.value);
            cell.value = updated_cell.value;
            text.sections[0].value = format!("{}", cell.value);
        }
    }
}

pub fn draw_grid(
    _commands: Commands,
    mut painter: ShapePainter,
    world_grid: Res<WorldGrid>,
    game_cursor: Res<GameCursor>,
    _asset_server: Res<AssetServer>,
) {
    let rotation = Quat::from_rotation_x(TAU * 0.25);
    painter.set_rotation(rotation);
    painter.thickness = 0.01;
    let cursor_grid_position = world_grid
        .get_grid_position_from_world_position(game_cursor.world_position.unwrap_or_default());
    for cell in &world_grid {
        let cell_selected = cursor_grid_position == cell.position;
        let mut position = world_grid.grid_to_world(&cell.position);

        painter.hollow = !cell_selected;
        painter.color = if cell_selected {
            Color::GRAY
        } else {
            Color::WHITE
        };

        position.y += 0.001;
        painter.transform.translation = position;
        // painter.circle(world_grid.grid_size * 0.5);
        painter.rect(Vec2::splat(world_grid.grid_size));
    }
}

pub fn gird_test_system(mut word_grid: ResMut<WorldGrid>) {
    word_grid.set_cell(Cell {
        position: GridPosition { x: 1, y: 1 },
        value: 5,
    })
}
