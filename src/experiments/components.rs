use crate::world_grid::components::GridPosition;
use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};

#[derive(Default, Component)]
pub struct TimeKeeper {
    pub elapsed_time: f32,
    pub delta_time: f32,
}

#[derive(Default, Clone, PartialEq, Eq, Hash, Debug, Component, Reflect)]
pub struct DebugPathNode {
    pub position: GridPosition,
}

#[derive(Default, Clone, PartialEq, Eq, Hash, Debug, Component, Reflect)]
pub struct PathNode {
    pub position: GridPosition,
    pub walking_cost: i32,
    pub heuristic_cost: i32,
    #[reflect(ignore)]
    pub previous_node: Option<Box<PathNode>>,
}

impl PathNode {
    pub fn combined_cost(&self) -> i32 {
        self.walking_cost + self.heuristic_cost
    }
}

#[derive(Event)]
pub struct PathRequestEvent {
    pub start: GridPosition,
    pub end: GridPosition,
}

#[derive(Event)]
pub struct PathFoundEvent {
    pub path: Vec<PathNode>,
}

#[derive(Default)]
pub struct InfinityPathingGrid {
    nodes: HashMap<GridPosition, PathNode>,
}

impl InfinityPathingGrid {
    pub fn get_node(&self, position: GridPosition) -> PathNode {
        self.nodes.get(&position).cloned().unwrap_or(PathNode {
            position,
            walking_cost: i32::MAX,
            heuristic_cost: 0,
            previous_node: None,
        })
    }

    pub fn add_node(&mut self, node: PathNode) {
        self.nodes.insert(node.position, node);
    }

    pub fn get_neighbours(&self, position: GridPosition) -> HashSet<PathNode> {
        let mut neighbour_positions = HashSet::new();

        neighbour_positions.insert(GridPosition {
            x: position.x - 1,
            y: position.y - 1,
        });

        neighbour_positions.insert(GridPosition {
            x: position.x,
            y: position.y - 1,
        });

        neighbour_positions.insert(GridPosition {
            x: position.x + 1,
            y: position.y - 1,
        });

        neighbour_positions.insert(GridPosition {
            x: position.x - 1,
            y: position.y,
        });

        neighbour_positions.insert(GridPosition {
            x: position.x + 1,
            y: position.y,
        });

        neighbour_positions.insert(GridPosition {
            x: position.x - 1,
            y: position.y + 1,
        });

        neighbour_positions.insert(GridPosition {
            x: position.x,
            y: position.y + 1,
        });

        neighbour_positions.insert(GridPosition {
            x: position.x + 1,
            y: position.y + 1,
        });

        let mut neighbours = HashSet::new();

        for &neighbour_position in &neighbour_positions {
            let node = self.get_node(neighbour_position);
            neighbours.insert(node);
        }

        neighbours
    }
}
