use bevy::prelude::*;

use crate::logic::cell::Cell;

use super::board::{GameState, COLUMNS, ROWS, SLOT_SIZE, SLOT_SPACE_BETWEEN};

#[derive(Component, Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum ShipType {
    Submarine,
    Battleship,
    LargeBattleship,
}

#[derive(Component, Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum ShipDirection {
    Horizontal,
    Vertical,
}

#[derive(Component, Clone, Debug)]
#[allow(dead_code)]
pub struct Ship {
    pub r#type: ShipType,
    pub cells: Vec<Entity>,
    pub sunk: bool,
}

#[derive(Bundle)]
#[allow(dead_code)]
pub struct ShipBundle {
    pub ship: Ship,
    pub direction: ShipDirection,
    pub transform: Transform,
    pub sprite: Sprite,
}

pub const SUBMARINE_SIZE: usize = 1;
pub const BATTLESHIP_SIZE: usize = 3;
pub const LARGE_BATTLESHIP_SIZE: usize = 4;

#[allow(dead_code)]
impl ShipBundle {
    pub fn new_submarine(
        asset_server: &Res<AssetServer>,
        direction: ShipDirection,
        x: i8,
        y: i8,
        cells_query: &Query<(Entity, &Cell)>,
        game_state: &mut ResMut<GameState>,
    ) -> ShipBundle {
        let color = if y > 4 {
            game_state.total_ships_bot += 1;
            Color::srgba(1.0, 1.0, 1.0, 0.0) // Azul claro para inimigos
        } else {
            game_state.total_ships_player += 1;
            Color::srgba(1.0, 1.0, 1.0, 1.0) // Cor padrão para o jogador
        };

        ShipBundle {
            ship: Ship {
                r#type: ShipType::Submarine,
                cells: Self::find_cells_for_ship(x, y, SUBMARINE_SIZE, &direction, &cells_query),
                sunk: false,
            },
            direction: direction.clone(),
            transform: Transform {
                translation: ShipBundle::calculate_position(SUBMARINE_SIZE, &direction, x, y),
                rotation: match direction {
                    ShipDirection::Horizontal => Quat::from_rotation_z(0.0),
                    ShipDirection::Vertical => Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                },
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(SUBMARINE_SIZE as f32 * SLOT_SIZE, SLOT_SIZE)),
                image: asset_server.load("atlases/submarine.png"),
                color,
                ..Default::default()
            },
        }
    }

    pub fn new_battleship(
        asset_server: &Res<AssetServer>,
        direction: ShipDirection,
        x: i8,
        y: i8,
        cells_query: &Query<(Entity, &Cell)>,
        game_state: &mut ResMut<GameState>,
    ) -> ShipBundle {
        let color = if y > 4 {
            game_state.total_ships_bot += 1;
            Color::srgba(1.0, 1.0, 1.0, 0.0) // Azul claro para inimigos
        } else {
            game_state.total_ships_player += 1;
            Color::srgba(1.0, 1.0, 1.0, 1.0) // Cor padrão para o jogador
        };

        ShipBundle {
            ship: Ship {
                r#type: ShipType::Battleship,
                cells: Self::find_cells_for_ship(x, y, BATTLESHIP_SIZE, &direction, &cells_query),
                sunk: false,
            },
            direction: direction.clone(),
            transform: Transform {
                translation: ShipBundle::calculate_position(BATTLESHIP_SIZE, &direction, x, y),
                rotation: match direction {
                    ShipDirection::Horizontal => Quat::from_rotation_z(0.0),
                    ShipDirection::Vertical => Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                },
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(BATTLESHIP_SIZE as f32 * SLOT_SIZE, SLOT_SIZE)),
                image: asset_server.load("atlases/battleship.png"),
                color,
                ..Default::default()
            },
        }
    }

    pub fn new_large_battleship(
        asset_server: &Res<AssetServer>,
        direction: ShipDirection,
        x: i8,
        y: i8,
        cells_query: &Query<(Entity, &Cell)>,
        game_state: &mut ResMut<GameState>,
    ) -> ShipBundle {

        let color = if y > 4 {
            game_state.total_ships_bot += 1;
            Color::srgba(1.0, 1.0, 1.0, 0.0) // Azul claro para inimigos
        } else {
            game_state.total_ships_player += 1;
            Color::srgba(1.0, 1.0, 1.0, 1.0) // Cor padrão para o jogador
        };

        ShipBundle {
            ship: Ship {
                r#type: ShipType::LargeBattleship,
                cells: Self::find_cells_for_ship(
                    x,
                    y,
                    LARGE_BATTLESHIP_SIZE,
                    &direction,
                    &cells_query,
                ),
                sunk: false,
            },
            direction: direction.clone(),
            transform: Transform {
                translation: ShipBundle::calculate_position(
                    LARGE_BATTLESHIP_SIZE,
                    &direction,
                    x,
                    y,
                ),
                rotation: match direction {
                    ShipDirection::Horizontal => Quat::from_rotation_z(0.0),
                    ShipDirection::Vertical => Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
                },
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    LARGE_BATTLESHIP_SIZE as f32 * SLOT_SIZE,
                    SLOT_SIZE,
                )),
                image: asset_server.load("atlases/large_battleship.png"),
                color,
                ..Default::default()
            },
        }
    }

    pub fn calculate_position(ship_size: usize, direction: &ShipDirection, x: i8, y: i8) -> Vec3 {
        let x = x as f32;
        let y = y as f32;

        Vec3::new(
            if ship_size % 2 == 0 && direction == &ShipDirection::Horizontal {
                (x) * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                    - (ROWS as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN) / 2.0)
                    + (SLOT_SIZE * 1.5)
            } else {
                (x) * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                    - (ROWS as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN) / 2.0)
            },
            if ship_size % 2 == 0 && direction == &ShipDirection::Vertical {
                (y) * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                    - (COLUMNS as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN) / 2.0)
                    + (SLOT_SIZE * 1.5)
            } else {
                (y) * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                    - (COLUMNS as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN) / 2.0)
            },
            Vec3::default().z + 1.0,
        )
    }

    fn find_cells_for_ship(
        x: i8,
        y: i8,
        ship_size: usize,
        direction: &ShipDirection,
        cells_query: &Query<(Entity, &Cell)>,
    ) -> Vec<Entity> {
        let mut cells = Vec::new();

        for i in 0..ship_size {
            let target_x = if *direction == ShipDirection::Horizontal {
                x + i as i8
            } else {
                x
            };
            let target_y = if *direction == ShipDirection::Vertical {
                y + i as i8
            } else {
                y
            };

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
    mut game_state: ResMut<GameState>,
) {
    //peça do bot
    commands.spawn(ShipBundle::new_large_battleship(
        &asset_server,
        ShipDirection::Vertical,
        4,
        5,
        &cells_query,
        &mut game_state,
    ));

    // commands.spawn(ShipBundle::new_battleship(
    //     &asset_server,
    //     ShipDirection::Vertical,
    //     6,
    //     6,
    //     &cells_query,
    //     &mut game_state,
    // ));

    commands.spawn(ShipBundle::new_submarine(
        &asset_server,
        ShipDirection::Vertical,
        9,
        7,
        &cells_query,
        &mut game_state,
    ));

    //peça do jogador
    commands.spawn(ShipBundle::new_large_battleship(
        &asset_server,
        ShipDirection::Vertical,
        9,
        1,
        &cells_query,
        &mut game_state,
    ));

    // commands.spawn(ShipBundle::new_battleship(
    //     &asset_server,
    //     ShipDirection::Horizontal,
    //     4,
    //     1,
    //     &cells_query,
    //     &mut game_state,
    // ));

    commands.spawn(ShipBundle::new_submarine(
        &asset_server,
        ShipDirection::Horizontal,
        7,
        3,
        &cells_query,
        &mut game_state,
    ));


}

// função que vai escutar a mudança de sunk e vai fazer algo a partir disso
pub fn check_sunk_change(mut query: Query<(&mut Ship, &mut Sprite)>) {
    for (ship, mut sprite) in query.iter_mut() {
        if ship.sunk {
            sprite.color = Color::srgba(1.0, 1.0, 1.0, 1.0);
        }
    }
}
