use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::logic::cell::{Cell, CellSide};

use super::{
    board::{GameState, PLAYER_CELL_COLOR, SLOT_SIZE, SLOT_SPACE_BETWEEN},
    ships::{
        Ship, ShipBundle, ShipDirection, ShipType, AIRCRAFT_CARRIER_SIZE, BATTLESHIP_SIZE, LARGE_BATTLESHIP_SIZE, SUBMARINE_SIZE
    },
};

#[derive(Component)]
#[allow(dead_code)]
pub struct ShipSelectionPanel;

#[derive(Component, Debug)]
#[allow(dead_code)]
pub struct ShipOption {
    pub ship_type: ShipType,
}

#[derive(Component, Debug)]
#[allow(dead_code)]
pub struct SelectedShip(pub ShipType);

#[allow(dead_code)]
impl Plugin for ShipSelectionPanel {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup_ship_selection_panel)
            .add_systems(
                Update,
                (
                    handle_ship_selection_button_drag,
                    handle_selected_ship_translation_with_cursor,
                    handle_selected_ship_button_drop,
                ),
            );
    }
}

const OPTIONS_BORDER_COLOR: Color = Color::srgb(0.172, 0.282, 0.561);
const OPTIONS_BACKGROUND_COLOR: Color = Color::srgb(0.224, 0.451, 0.678);
const OPTIONS_BORDER_RADIUS: f32 = 8.0;
const OPTIONS_BORDER_WIDTH: f32 = 2.0;

#[derive(Bundle)]
pub struct OptionButtonUI {
    button: Button,
    node: Node,
    border_radius: BorderRadius,
    border_color: BorderColor,
    background_color: BackgroundColor,
}

impl OptionButtonUI {
    pub fn new() -> Self {
        OptionButtonUI {
            button: Button,
            node: Node {
                width: Val::Px(SLOT_SIZE * AIRCRAFT_CARRIER_SIZE as f32),
                height: Val::Px(SLOT_SIZE),
                border: UiRect::all(Val::Px(OPTIONS_BORDER_WIDTH)),
                ..Default::default()
            },
            border_radius: BorderRadius::all(Val::Px(OPTIONS_BORDER_RADIUS)),
            border_color: BorderColor(OPTIONS_BORDER_COLOR),
            background_color: BackgroundColor(OPTIONS_BACKGROUND_COLOR),
        }
    }
}



fn setup_ship_selection_panel(
    assert_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(5.0),
            left: Val::Percent(5.0),
            width: Val::Px(150.0),
            height: Val::Px(150.0),
            border: UiRect::all(Val::Px(2.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(OptionButtonUI::new())
                .insert(ShipOption {
                    ship_type: ShipType::Submarine,
                })
                .with_children(|parent| {
                    parent.spawn(ImageNode::new(assert_server.load("atlases/submarine.png")));
                });
            parent
                .spawn(OptionButtonUI::new())
                .insert(ShipOption {
                    ship_type: ShipType::Battleship,
                })
                .with_children(|parent| {
                    parent.spawn(ImageNode::new(assert_server.load("atlases/battleship.png")));
                });
            parent
                .spawn(OptionButtonUI::new())
                .insert(ShipOption {
                    ship_type: ShipType::LargeBattleship,
                })
                .with_children(|parent| {
                    parent.spawn(ImageNode::new(
                        assert_server.load("atlases/large_battleship.png"),
                    ));
                });
            parent
                .spawn(OptionButtonUI::new())
                .insert(ShipOption {
                    ship_type: ShipType::AircraftCarrier,
                })
                .with_children(|parent| {
                    parent.spawn(ImageNode::new(
                        assert_server.load("atlases/aircraft_carrier.png"),
                    ));
                });
        });
}

fn handle_ship_selection_button_drag(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut interaction_query: Query<
        (&Button, &Interaction, &ShipOption, &mut Transform),
        (Changed<Interaction>, With<Button>),
    >,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    cells_query: Query<(Entity, &Cell)>,
    selected_ship_query: Query<Entity, With<SelectedShip>>,
    mut game_state: ResMut<GameState>,
) {
    for (_, interaction, ship_option, mut transform) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                transform.scale = Vec3::splat(0.99);

                if let Ok(selected_ship_entity) = selected_ship_query.get_single() {
                    commands.entity(selected_ship_entity).despawn_recursive();
                }

                let mut window = window_query.single_mut();

                window.cursor_options.grab_mode = CursorGrabMode::None;
                window.cursor_options.visible = true;

                commands.spawn((
                    SelectedShip(ship_option.ship_type.clone()),
                    match ship_option.ship_type {
                        ShipType::Submarine => ShipBundle::new_submarine(
                            &asset_server,
                            ShipDirection::Horizontal,
                            0,
                            0,
                            &cells_query,
                            &mut game_state,
                        ),
                        ShipType::Battleship => ShipBundle::new_battleship(
                            &asset_server,
                            ShipDirection::Horizontal,
                            0,
                            0,
                            &cells_query,
                            &mut game_state,
                        ),
                        ShipType::LargeBattleship => ShipBundle::new_large_battleship(
                            &asset_server,
                            ShipDirection::Horizontal,
                            0,
                            0,
                            &cells_query,
                            &mut game_state,
                        ),
                        ShipType::AircraftCarrier => ShipBundle::new_aircraft_carrier(
                            &asset_server,
                            ShipDirection::Horizontal,
                            0,
                            0,
                            &cells_query,
                            &mut game_state,
                        ),
                    },
                ));
            }
            Interaction::None => {
                transform.scale = Vec3::splat(1.0);
            }
            Interaction::Hovered => {
                transform.scale = Vec3::splat(1.01);
            }
        }
    }
}

fn handle_selected_ship_translation_with_cursor(
    mut selected_ship_query: Query<(&SelectedShip, &ShipDirection, &mut Transform), Without<Cell>>,
    mut cells_query: Query<(&mut Sprite, &Transform, &CellSide), With<Cell>>,
    window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let x = cursor_position.x - window.width() / 2.0;
    let y = (window.height() - cursor_position.y) - window.height() / 2.0;

    let cursor_world_position = Vec3::new(x, y, Vec3::default().z);

    let Some((selected_ship, ship_direction, mut ship_transform)) =
        selected_ship_query.iter_mut().next()
    else {
        return;
    };

    ship_transform.translation = cursor_world_position;

    // when the cursor is over the board, change covered cells color

    for (mut cell_sprite, cell_transform, cell_side) in cells_query.iter_mut() {
        if cell_side == &CellSide::Enemy {
            continue;
        }

        let x_range = match selected_ship.0 {
            ShipType::Submarine => {
                if ship_direction == &ShipDirection::Horizontal {
                    SUBMARINE_SIZE as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                } else {
                    SLOT_SIZE
                }
            }
            ShipType::Battleship => {
                if ship_direction == &ShipDirection::Horizontal {
                    BATTLESHIP_SIZE as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                } else {
                    SLOT_SIZE
                }
            }
            ShipType::LargeBattleship => {
                if ship_direction == &ShipDirection::Horizontal {
                    LARGE_BATTLESHIP_SIZE as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                } else {
                    SLOT_SIZE
                }
            }
            ShipType::AircraftCarrier => {
                if ship_direction == &ShipDirection::Horizontal {
                    AIRCRAFT_CARRIER_SIZE as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                } else {
                    SLOT_SIZE
                }
            }
        } / 2.0;

        let y_range = match selected_ship.0 {
            ShipType::Submarine => {
                if ship_direction == &ShipDirection::Vertical {
                    SUBMARINE_SIZE as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                } else {
                    SLOT_SIZE
                }
            }
            ShipType::Battleship => {
                if ship_direction == &ShipDirection::Vertical {
                    BATTLESHIP_SIZE as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                } else {
                    SLOT_SIZE
                }
            }
            ShipType::LargeBattleship => {
                if ship_direction == &ShipDirection::Vertical {
                    LARGE_BATTLESHIP_SIZE as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                } else {
                    SLOT_SIZE
                }
            }
            ShipType::AircraftCarrier => {
                if ship_direction == &ShipDirection::Vertical {
                    AIRCRAFT_CARRIER_SIZE as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                } else {
                    SLOT_SIZE
                }
            }
        } / 2.0;

        let x1 = cell_transform.translation.x - x_range;
        let x2 = cell_transform.translation.x + x_range;
        let y1 = cell_transform.translation.y - y_range;
        let y2 = cell_transform.translation.y + y_range;

        if x >= x1 && x <= x2 && y >= y1 && y <= y2 {
            cell_sprite.color = Color::WHITE;
        } else {
            cell_sprite.color = PLAYER_CELL_COLOR;
        }
    }
}

fn handle_selected_ship_button_drop(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut selected_ship_query: Query<
        (
            Entity,
            &mut Ship,
            &SelectedShip,
            &ShipDirection,
            &mut Transform,
            &mut Sprite,
        ),
    >,
    mut cells_query: Query<
        (Entity, &Transform, &mut Sprite, &Cell, &CellSide),
        Without<SelectedShip>,
    >,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = window_query.single_mut();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let window_x = cursor_position.x - window.width() / 2.0;
    let window_y = (window.height() - cursor_position.y) - window.height() / 2.0;

    let Ok((
        selected_ship_entity,
        mut ship,
        selected_ship,
        ship_direction,
        mut ship_transform,
        mut _ship_sprite,
    )) = selected_ship_query.get_single_mut()
    else {
        return;
    };

    if mouse_button_input.just_released(MouseButton::Right)
        && window.cursor_options.grab_mode == CursorGrabMode::None
    {
        let x_range = match selected_ship.0 {
            ShipType::Submarine => {
                if ship_direction == &ShipDirection::Horizontal {
                    SUBMARINE_SIZE as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                } else {
                    SLOT_SIZE
                }
            }
            ShipType::Battleship => {
                if ship_direction == &ShipDirection::Horizontal {
                    BATTLESHIP_SIZE as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                } else {
                    SLOT_SIZE
                }
            }
            ShipType::LargeBattleship => {
                if ship_direction == &ShipDirection::Horizontal {
                    LARGE_BATTLESHIP_SIZE as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                } else {
                    SLOT_SIZE
                }
            }
            ShipType::AircraftCarrier => {
                if ship_direction == &ShipDirection::Horizontal {
                    AIRCRAFT_CARRIER_SIZE as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                } else {
                    SLOT_SIZE
                }
            }
        } / 2.0;

        let y_range = match selected_ship.0 {
            ShipType::Submarine => {
                if ship_direction == &ShipDirection::Vertical {
                    SUBMARINE_SIZE as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                } else {
                    SLOT_SIZE
                }
            }
            ShipType::Battleship => {
                if ship_direction == &ShipDirection::Vertical {
                    BATTLESHIP_SIZE as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                } else {
                    SLOT_SIZE
                }
            }
            ShipType::LargeBattleship => {
                if ship_direction == &ShipDirection::Vertical {
                    LARGE_BATTLESHIP_SIZE as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                } else {
                    SLOT_SIZE
                }
            }
            ShipType::AircraftCarrier => {
                if ship_direction == &ShipDirection::Vertical {
                    AIRCRAFT_CARRIER_SIZE as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                } else {
                    SLOT_SIZE
                }
            }
        } / 2.0;

        let mut cells_entities_and_data: Vec<(Entity, &Cell)> = Vec::new();

        for (cell_entity, cell_transform, mut cell_sprite, cell, cell_side) in cells_query.iter_mut() {
            if cell_side == &CellSide::Enemy {
                continue;
            }

            let x1 = cell_transform.translation.x - x_range;
            let x2 = cell_transform.translation.x + x_range;
            let y1 = cell_transform.translation.y - y_range;
            let y2 = cell_transform.translation.y + y_range;

            if window_x >= x1 && window_x <= x2 && window_y >= y1 && window_y <= y2 {
                cell_sprite.color = Color::srgb(0.0, 1.0, 0.0);
                println!("Selected cell: {:?}", cell);
                cells_entities_and_data.push((cell_entity, cell));
            }
        }

        if ship.cells.len() == 0 {
            commands.entity(selected_ship_entity).despawn_recursive();
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
            return;
        }

        commands
            .entity(selected_ship_entity)
            .remove::<SelectedShip>();

        cells_entities_and_data.sort_by(|(_, a), (_, b)| {
            if a.row == b.row {
                a.column.cmp(&b.column)
            } else {
                a.row.cmp(&b.row)
            }
        });

        let final_ship_position = ShipBundle::calculate_position(
            match selected_ship.0 {
                ShipType::Submarine => SUBMARINE_SIZE,
                ShipType::Battleship => BATTLESHIP_SIZE,
                ShipType::LargeBattleship => LARGE_BATTLESHIP_SIZE,
                ShipType::AircraftCarrier => AIRCRAFT_CARRIER_SIZE,
            },
            ship_direction,
            cells_entities_and_data[0].1.column as i8,
            cells_entities_and_data[0].1.row as i8,
        );

        ship_transform.translation.x = final_ship_position.x;
        ship_transform.translation.y = final_ship_position.y;
        ship_transform.translation.z = final_ship_position.z;

        ship.cells = cells_entities_and_data
            .iter()
            .map(|(cell_entity, _)| *cell_entity)
            .collect();

        //mudar cor do navio caso ele esteja numa posição inimiga (descomentar quando a funcionalidade do inimigo posicionar navio estiver funcionando)
        // if let Some(last_cell) = cells.last() {

        //     let new_color = if last_cell.row >= 4 {
        //         Color::srgb(0.6, 0.6, 1.0) // Azul claro para inimigos
        //     } else {
        //         Color::srgb(1.0, 1.0, 1.0) // Cor padrão (branca) para o jogador
        //     };

        //     ship_sprite.color = new_color;
        // }

        for (_, _, mut cell_sprite, _, cell_side) in cells_query.iter_mut() {
            if cell_side == &CellSide::Player {
                cell_sprite.color = PLAYER_CELL_COLOR;
            }
        }

        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.cursor_options.visible = true;
    }
}
