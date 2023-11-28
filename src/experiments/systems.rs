use std::f32::consts::TAU;
use std::ops::Range;

use crate::experiments::components::{PathRequestEvent, TimeKeeper};
use crate::world_grid::components::{GridPosition, WorldGrid};
use bevy::prelude::*;
use bevy::time::Time;
use bevy::utils::HashSet;
use bevy_mod_billboard::BillboardTextBundle;
use bevy_turborand::prelude::*;
use bevy_vector_shapes::prelude::ShapeCommands;
use bevy_vector_shapes::shapes::RectangleSpawner;

use super::components::{DebugPathNode, InfinityPathingGrid, PathFoundEvent, PathNode};

#[allow(dead_code)]
pub fn limited_rate_system(time: Res<Time>, mut time_keeper: Local<TimeKeeper>) {
    time_keeper.delta_time = time.elapsed_seconds() - time_keeper.elapsed_time;
    time_keeper.elapsed_time = time.elapsed_seconds();
    println!("Time elapsed: {}", time_keeper.delta_time)
}

pub fn input_for_testing_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut path_event: EventWriter<PathRequestEvent>,
    mut global_rng: ResMut<GlobalRng>,
    world_grid: Res<WorldGrid>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        path_event.send(PathRequestEvent {
            start: GridPosition {
                x: global_rng.i32(0..world_grid.width),
                y: global_rng.i32(0..world_grid.height),
            },
            end: GridPosition {
                x: global_rng.i32(0..world_grid.width),
                y: global_rng.i32(0..world_grid.height),
            },
        });
    }
}

pub fn find_path_system(
    mut path_requests: EventReader<PathRequestEvent>,
    mut path_found_writer: EventWriter<PathFoundEvent>,
    // world_grid: Res<WorldGrid>,
) {
    for path_request in path_requests.read() {
        let start_node = path_request.start;
        let end_node = path_request.end;
        let mut path_grid = InfinityPathingGrid::default();
        let mut open_list = HashSet::<PathNode>::new();

        let mut closed_list = HashSet::<PathNode>::new();

        let start_node = PathNode {
            position: start_node,
            walking_cost: 0,
            heuristic_cost: calculate_distance_cost(path_request.start, path_request.end),
            previous_node: None,
        };

        open_list.insert(start_node);

        while !open_list.is_empty() {
            let current_node = get_lowest_cost_node(&open_list);

            if let Some(current_node) = current_node {
                if current_node.position == end_node {
                    let full_path = calculate_path(current_node);
                    println!("we found the path!");
                    path_found_writer.send(PathFoundEvent { path: full_path });
                    return;
                }

                open_list.remove(&current_node);
                closed_list.insert(current_node.clone());

                for mut node in path_grid.get_neighbours(current_node.position) {
                    if closed_list.contains(&node) {
                        continue;
                    };

                    let tentative_walking_cost = current_node.walking_cost
                        + calculate_distance_cost(current_node.position, node.position);
                    if tentative_walking_cost < node.walking_cost {
                        node.previous_node = Some(Box::new(current_node.clone()));
                        node.walking_cost = tentative_walking_cost;
                        node.heuristic_cost = calculate_distance_cost(node.position, end_node);

                        path_grid.add_node(node.clone());

                        if !open_list.contains(&node) {
                            open_list.insert(node);
                        }
                    }
                }
            }
        }
    }
}

const STRAIGHT_MOVEMENT_COST: i32 = 10;
const DIAGONAL_MOVEMENT_COST: i32 = 14;

fn calculate_distance_cost(a: GridPosition, b: GridPosition) -> i32 {
    let x_distance = (a.x - b.x).abs();
    let y_distance = (a.y - b.y).abs();
    let remaining = (x_distance - y_distance).abs();
    x_distance.min(y_distance) * DIAGONAL_MOVEMENT_COST + remaining * STRAIGHT_MOVEMENT_COST
}

fn get_lowest_cost_node(path_nodes: &HashSet<PathNode>) -> Option<PathNode> {
    path_nodes.iter().min_by_key(|p| p.combined_cost()).cloned()
}

fn calculate_path(end_node: PathNode) -> Vec<PathNode> {
    let mut path = Vec::new();

    path.push(end_node.clone());
    let mut current_node = end_node.clone();

    while current_node.previous_node.is_some() {
        path.push(*current_node.previous_node.clone().unwrap());
        current_node = *current_node.previous_node.unwrap();
    }

    path.reverse();
    path
}

pub fn debug_path_finding(
    mut commands: Commands,
    mut shapes: ShapeCommands,
    mut path_found_event: EventReader<PathFoundEvent>,
    world_grid: Res<WorldGrid>,
    asset_server: Res<AssetServer>,
    existing_debug_q: Query<Entity, With<DebugPathNode>>,
) {
    if path_found_event.is_empty() {
        return;
    }
    for e in existing_debug_q.iter() {
        commands.entity(e).despawn();
    }

    for path_event in path_found_event.read() {
        for node in &path_event.path {
            let mut position = world_grid.grid_to_world(&node.position);
            position.y += 0.15;
            commands
                .spawn((
                    BillboardTextBundle {
                        transform: Transform::from_translation(position)
                            // .with_rotation(rotation)
                            .with_scale(Vec3::splat(0.008)),
                        text: Text::from_sections([TextSection {
                            value: format!(
                                "w:{}\nh:{}\nt:{}",
                                node.walking_cost,
                                node.heuristic_cost,
                                node.combined_cost()
                            ),
                            style: TextStyle {
                                font_size: 30.0,
                                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                color: Color::WHITE,
                            },
                        }])
                        .with_alignment(TextAlignment::Center),
                        ..default()
                    },
                    node.position,
                ))
                .insert(DebugPathNode {
                    position: node.position,
                });
            shapes.color = Color::BLACK;
            shapes
                .rect(Vec2::new(1.0, 1.0))
                .insert(
                    Transform::from_translation(position - Vec3::Y * 0.01)
                        .with_rotation(Quat::from_rotation_x(TAU * 0.25)),
                )
                .insert(DebugPathNode {
                    position: node.position,
                });
        }
    }
}
