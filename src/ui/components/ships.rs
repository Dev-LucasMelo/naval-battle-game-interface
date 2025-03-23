use bevy::prelude::*;

use crate::logic::cell::Cell;

use super::board::{COLUMNS, ROWS, SLOT_SIZE, SLOT_SPACE_BETWEEN};

#[derive(Component, Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Component, Clone)]
#[allow(dead_code)]
pub struct Ship {
    pub direction: Direction,
    pub cells: Vec<Entity>,
    pub sunk: bool,
}

#[derive(Bundle)]
#[allow(dead_code)]
pub struct ShipBundle {
    pub ship: Ship,
    pub transform: Transform,
    pub sprite: Sprite,
}

const SUBMARINE_SIZE: usize = 1;
const BATTLESHIP_SIZE: usize = 3;
const LARGE_BATTLESHIP_SIZE: usize = 4;

#[allow(dead_code)]
impl ShipBundle {
    pub fn new_submarine(
        asset_server: &Res<AssetServer>,
        direction: Direction,
        x: i8,
        y: i8,
        cells_query: &Query<(Entity, &Cell)>,
    ) -> ShipBundle {
        ShipBundle {
            ship: Ship {
                direction: direction.clone(),
                cells: Self::find_cells_for_ship(x, y, SUBMARINE_SIZE, &direction, &cells_query),
                sunk: false,
            },
            transform: Transform {
                translation: ShipBundle::calculate_position(SUBMARINE_SIZE, &direction, x, y),
                rotation: match direction {
                    Direction::Horizontal => Quat::from_rotation_z(0.0),
                    Direction::Vertical => Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                },
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(SUBMARINE_SIZE as f32 * SLOT_SIZE, SLOT_SIZE)),
                image: asset_server.load("atlases/submarine.png"),
                ..Default::default()
            },
        }
    }

    pub fn new_battleship(
        asset_server: &Res<AssetServer>,
        direction: Direction,
        x: i8,
        y: i8,
        cells_query: &Query<(Entity, &Cell)>,
    ) -> ShipBundle {
        ShipBundle {
            ship: Ship {
                direction: direction.clone(),
                cells: Self::find_cells_for_ship(x, y, BATTLESHIP_SIZE, &direction, &cells_query),
                sunk: false,
            },
            transform: Transform {
                translation: ShipBundle::calculate_position(BATTLESHIP_SIZE, &direction, x, y),
                rotation: match direction {
                    Direction::Horizontal => Quat::from_rotation_z(0.0),
                    Direction::Vertical => Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                },
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(BATTLESHIP_SIZE as f32 * SLOT_SIZE, SLOT_SIZE)),
                image: asset_server.load("atlases/battleship.png"),
                ..Default::default()
            },
        }
    }

    pub fn new_large_battleship(
        asset_server: &Res<AssetServer>,
        direction: Direction,
        x: i8,
        y: i8,
        cells_query: &Query<(Entity, &Cell)>,
    ) -> ShipBundle {
        ShipBundle {
            ship: Ship {
                direction: direction.clone(),
                cells: Self::find_cells_for_ship(x, y, LARGE_BATTLESHIP_SIZE, &direction, &cells_query),
                sunk: false,
            },
            transform: Transform {
                translation: ShipBundle::calculate_position(LARGE_BATTLESHIP_SIZE, &direction, x, y),
                rotation: match direction {
                    Direction::Horizontal => Quat::from_rotation_z(0.0),
                    Direction::Vertical => Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                },
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(LARGE_BATTLESHIP_SIZE as f32 * SLOT_SIZE, SLOT_SIZE)),
                image: asset_server.load("atlases/large_battleship.png"),
                ..Default::default()
            },
        }
    }

    pub fn calculate_position(
        ship_size: usize,
        direction: &Direction,
        x: i8,
        y: i8,
    ) -> Vec3 {
        let x = x as f32;
        let y = y as f32;

        Vec3::new(
            if ship_size % 2 == 0 && direction == &Direction::Horizontal {
                (x as f32) * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                - (ROWS as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN) / 2.0)
                + (SLOT_SIZE * 1.5)
            } else {
                (x as f32) * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                - (ROWS as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN) / 2.0)
            },
            if ship_size % 2 == 0 && direction == &Direction::Vertical {
                (y as f32) * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                - (COLUMNS as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN) / 2.0)
                + (SLOT_SIZE * 1.5)
            } else {
                (y as f32) * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                - (COLUMNS as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN) / 2.0)
            },
            2.0,
        )
    }

    fn find_cells_for_ship(
        x: i8,
        y: i8,
        ship_size: usize,
        direction: &Direction,
        cells_query: &Query<(Entity, &Cell)>
    ) -> Vec<Entity> {
        let mut cells = Vec::new();

        for i in 0..ship_size {
            let target_x = if *direction == Direction::Horizontal { x + i as i8 } else { x };
            let target_y = if *direction == Direction::Vertical { y + i as i8 } else { y };

            if let Some((entity, _)) = cells_query
                .iter()
                .find(|(_, cell)| cell.column == target_x as usize && cell.row == target_y as usize)
            {
                cells.push(entity);
            }
        }

        cells
    }
}

pub fn debug_spawn_submarine(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cells_query: Query<(Entity, &Cell)>,
) {
    commands.spawn(
        ShipBundle::new_large_battleship(
            &asset_server,
            Direction::Vertical,
            1,
            1,
            &cells_query,
        ),
    );
}
