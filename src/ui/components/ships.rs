use bevy::prelude::*;

use super::board::{SLOT_SIZE, SLOT_SPACE_BETWEEN};

#[derive(Component, Clone)]
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
}

pub fn debug_spawn_submarine(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(
        ShipBundle::new_submarine(
            &asset_server,
            Direction::Horizontal,
    Vec3::new(0.0, 0.0, 2.0),
            Vec::new(),
        ),
    );

    commands.spawn(
        ShipBundle::new_battleship(
            &asset_server,
            Direction::Vertical,
            Vec3::new(SLOT_SIZE * 2.0 + SLOT_SPACE_BETWEEN * 2.0, 0.0, 2.0),
            Vec::new(),
        ),
    );

    commands.spawn(
        ShipBundle::new_battleship(
            &asset_server,
            Direction::Horizontal,
            Vec3::new(SLOT_SIZE * -4.0 + SLOT_SPACE_BETWEEN * -4.0, 0.0, 2.0),
            Vec::new(),
        ),
    );
}
