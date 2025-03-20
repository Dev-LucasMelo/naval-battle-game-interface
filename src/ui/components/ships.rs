use bevy::prelude::*;

use super::board::{SLOT_SIZE, SLOT_SPACE_BETWEEN};

#[derive(Component, Clone, PartialEq)]
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

impl ShipBundle {
    pub fn new_submarine(
        asset_server: &Res<AssetServer>,
        direction: Direction,
        translation: Vec3,
        cells: Vec<Entity>,
    ) -> ShipBundle {
        ShipBundle {
            ship: Ship {
                direction: direction.clone(),
                cells,
                sunk: false,
            },
            transform: Transform {
                translation,
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
        translation: Vec3,
        cells: Vec<Entity>,
    ) -> ShipBundle {
        ShipBundle {
            ship: Ship {
                direction: direction.clone(),
                cells,
                sunk: false,
            },
            transform: Transform {
                translation,
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
        translation: Vec3,
        cells: Vec<Entity>,
    ) -> ShipBundle {
        ShipBundle {
            ship: Ship {
                direction: direction.clone(),
                cells,
                sunk: false,
            },
            transform: Transform {
                translation,
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
                (SLOT_SIZE * x) + (SLOT_SPACE_BETWEEN * x) + (SLOT_SIZE / 2.0)
            } else {
                (SLOT_SIZE * x) + (SLOT_SPACE_BETWEEN * x)
            },
            if ship_size % 2 == 0 && direction == &Direction::Vertical {
                (SLOT_SIZE * y) + (SLOT_SPACE_BETWEEN * y) + (SLOT_SIZE / 2.0)
            } else {
                (SLOT_SIZE * y) + (SLOT_SPACE_BETWEEN * y)
            },
            2.0,
        )
    }
}

pub fn debug_spawn_submarine(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(
        ShipBundle::new_large_battleship(
            &asset_server,
            Direction::Horizontal,
            ShipBundle::calculate_position(LARGE_BATTLESHIP_SIZE, &Direction::Horizontal, 0, -4),
            Vec::new(),
        ),
    );
    commands.spawn(
        ShipBundle::new_battleship(
            &asset_server,
            Direction::Horizontal,
            ShipBundle::calculate_position(BATTLESHIP_SIZE, &Direction::Horizontal, 0, 0),
            Vec::new(),
        ),
    );
}
