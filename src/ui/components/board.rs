use crate::logic::cell::{Cell, CellSide};
use bevy::input::{mouse::MouseButtonInput, ButtonState};

use crate::ui::components::ships::Ship;
pub use bevy::prelude::*;
use rand::prelude::SliceRandom;

pub struct Board;

//struct que vai representar o total de cliques do jogo globalmente
#[derive(Default, Resource)]
pub struct ClickedCells {
    pub cells: Vec<Entity>, 
}

#[derive(Default, Resource)]
pub struct GameState {
    pub is_player_turn: bool,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            is_player_turn: true,
        }
    }
}

/**
 * criando um plugin bevy para criar o
 * tabuleiro a partir de uma função que vai ser chamada
 * a partir da inicialização do bevy
 */
impl Plugin for Board {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, render_board);
        app.add_systems(Update, handle_click);
        app.add_systems(Update, bot_turn);

        app.insert_resource(ClickedCells::default()); //adicionando struct como recurso global do bevy
        app.insert_resource(GameState::new());
    }
}

//configs globais do tabuleiro
pub const SLOT_SIZE: f32 = 60.0;
pub const SLOT_SPACE_BETWEEN: f32 = 4.0;

pub const ROWS: usize = 10;
pub const COLUMNS: usize = 10;
pub const ENEMY_CELL_COLOR: Color = Color::srgb(0.0, 0.2, 0.4);
pub const PLAYER_CELL_COLOR: Color = Color::srgb(0.4, 0.7, 1.0);

fn render_board(mut commands: Commands) {
    for row in 0..ROWS {
        for column in 0..COLUMNS {
            let x = (column as f32) * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                - (COLUMNS as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN) / 2.0);
            let y = (row as f32) * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                - (ROWS as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN) / 2.0);

            commands.spawn((
                Sprite {
                    color: if row < ROWS / 2 {
                        PLAYER_CELL_COLOR
                    } else {
                        ENEMY_CELL_COLOR
                    },
                    custom_size: Some(Vec2::new(SLOT_SIZE, SLOT_SIZE)),
                    ..Default::default()
                },
                Transform {
                    translation: Vec3::new(x, y, Vec3::default().z),
                    ..Default::default()
                },
                Cell {
                    column,
                    row,
                    marked: false,
                },
                if row < ROWS / 2 {
                    CellSide::Player
                } else {
                    CellSide::Enemy
                },
            ));
        }
    }
}

fn bot_turn(
    mut game_state: ResMut<GameState>,
    mut query: Query<(Entity, &mut Cell, &mut Sprite)>,
    mut ships_query: Query<(Entity, &mut Ship)>,
    mut clicked_cells: ResMut<ClickedCells>,
) {
    if !game_state.is_player_turn {
        let mut available_cells: Vec<Entity> = query
            .iter_mut()
            .filter(|(_, cell, _)| cell.row < (ROWS / 2) && !cell.marked) // Lado do jogador (linha maior que ROWS / 2)
            .filter_map(|(entity, cell, _)| {
                if !cell.marked && !clicked_cells.cells.contains(&entity) {
                    //fazendo com que o bot não escolha celulas já clicadas
                    Some(entity)
                } else {
                    None
                }
            })
            .collect();

        let mut rng = rand::thread_rng();
        available_cells.shuffle(&mut rng);

        if let Some(target_entity) = available_cells.first() {
            for (entity, mut cell, mut sprite) in query.iter_mut() {
                if entity == *target_entity {
                    cell.mark(
                        &mut sprite,
                        &mut ships_query,
                        entity,
                        &mut clicked_cells.cells,
                        &mut game_state,
                    );

                    break;
                }
            }
        }
    }
}

fn handle_click(
    mut query: Query<(Entity, &mut Sprite, &mut Cell, &Transform)>,
    mut mouse_button_input: EventReader<MouseButtonInput>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut ships_query: Query<(Entity, &mut Ship)>,
    mut clicked_cells: ResMut<ClickedCells>,
    mut game_state: ResMut<GameState>,
) {
    if game_state.is_player_turn {
        for event in mouse_button_input.read() {
            if event.button == MouseButton::Left && event.state == ButtonState::Pressed {
                let (camera, camera_transform) = *camera_query;

                let Ok(window) = windows.get_single() else {
                    return;
                };

                let Some(cursor_position) = window.cursor_position() else {
                    return;
                };

                let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position)
                else {
                    return;
                };

                for (entity, mut sprite, mut cell, _) in query.iter_mut() {
                    let column = cell.column;
                    let row = cell.row;

                    // Cálculo da posição da célula no tabuleiro
                    let x = (column as f32) * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                        - (COLUMNS as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN) / 2.0);
                    let y = (row as f32) * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                        - (ROWS as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN) / 2.0);

                    // area da celula (precisa refinar)
                    let cell_area = Rect {
                        min: Vec2::new(x - SLOT_SIZE / 2.0, y - SLOT_SIZE / 2.0),
                        max: Vec2::new(x + SLOT_SIZE / 2.0, y + SLOT_SIZE / 2.0),
                    };

                    if cell_area.contains(point.xy()) {
                        if cell.row > (ROWS / 2) - 1 && !cell.marked {
                            cell.mark(
                                &mut sprite,
                                &mut ships_query,
                                entity,
                                &mut clicked_cells.cells,
                                &mut game_state,
                            );
                        } else if cell.marked {
                            println!("celula já marcada posicao");
                        } else {
                            // println!("não é possivel selecionar essa celula");
                        }
                    }
                }
            }
        }
    }
}
