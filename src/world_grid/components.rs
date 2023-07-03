
use bevy::prelude::*;

use bevy::utils::HashMap;


#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Cell {
    pub position: GridPosition,
}

#[derive(Resource)]
pub struct GridCursor {
    pub entity: Entity,
    pub ui_position: Option<Vec2>,
    pub selected_cell: Option<Cell>,
    pub world_position: Option<Vec3>,
}


#[derive(Resource)]
pub struct WorldGrid {
    pub width: i32,
    pub height: i32,
    pub grid_size: f32,
    pub cells: HashMap<GridPosition, Cell>,
}


impl WorldGrid {
    pub fn new(width: i32, height: i32, grid_size: f32) -> WorldGrid {
        let mut cells = HashMap::<GridPosition, Cell>::new();
        for x in 0..width {
            for y in 0..height {
                let position = GridPosition {x, y};
                cells.insert(position.clone(), Cell {
                    position
                });
            }
        }
        Self {
            width,
            height,
            cells,
            grid_size
        }
    }

    pub fn grid_to_world(&self,  grid_position: &GridPosition) -> Vec3 {
        return Vec3::new(grid_position.x as f32 * self.grid_size, 0.0, grid_position.y as f32 * self.grid_size)
    }
}



impl<'a> IntoIterator for &'a WorldGrid {
    type Item = &'a Cell;
    type IntoIter = WorldGridIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        WorldGridIterator {
            world_grid: self,
            position: GridPosition{x: 0, y: 0}
        }
    }


}

pub struct WorldGridIterator<'a> {
    world_grid: &'a WorldGrid,
    position: GridPosition
}

impl<'a> Iterator for WorldGridIterator<'a> {
    type Item = &'a Cell;
    fn next(&mut self) -> Option<Self::Item> {

        let result = self.world_grid.cells.get(&self.position);
        self.position.x += 1;

        if self.position.x >= self.world_grid.width {
            self.position.y += 1;
            self.position.x = 0;
        }

        result
    }
}