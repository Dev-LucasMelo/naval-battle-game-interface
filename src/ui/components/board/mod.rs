use crate::logic::cell::Cell;
use bevy::input::{mouse::MouseButtonInput, ButtonState};

pub use bevy::prelude::*;

pub struct Board;

/**
 * criando um plugin bevy para criar o
 * tabuleiro a partir de uma função que vai ser chamada
 * a partir da inicialização do bevy
 */
impl Plugin for Board {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, render_board);
        app.add_systems(Update, handle_click);
    }
}

//configs globais do tabuleiro
const SLOT_SIZE: f32 = 50.0;
const SLOT_SPACE_BETWEEN: f32 = 4.0;

const ROWS: usize = 10;
const COLUMNS: usize = 10;

fn render_board(mut commands: Commands) {
    for row in 0..ROWS {
        for column in 0..COLUMNS {
            let color = if row < ROWS / 2 {
                //parte do jogador atual
                Color::srgb(0.4, 0.7, 1.0)
            } else {
                //parte do inimigo
                Color::srgb(0.0, 0.2, 0.4)
            };

            let x = (column as f32) * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                - (COLUMNS as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN) / 2.0);
            let y = (row as f32) * (SLOT_SIZE + SLOT_SPACE_BETWEEN)
                - (ROWS as f32 * (SLOT_SIZE + SLOT_SPACE_BETWEEN) / 2.0);

            // Configuracoes do bevy para setar cor tamanho e posicionamento (aplicação de translação)
            commands
                .spawn(Sprite {
                    color,
                    custom_size: Some(Vec2::new(SLOT_SIZE, SLOT_SIZE)), //cria quadrados
                    ..Default::default()
                })
                .insert(
                    Transform {
                        translation: Vec3::new(x, y, 0.0), // Posição centralizada
                        ..Default::default()
                    }, // posicao do tabuleiro
                )
                .insert(Cell {
                    column,
                    row,
                    marked: false,
                });
        }
    }
}

fn handle_click(
    mut query: Query<(&mut Sprite, &mut Cell, &Transform)>,
    mut mouse_button_input: EventReader<MouseButtonInput>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    for event in mouse_button_input.read() {
        if event.button == MouseButton::Left && event.state == ButtonState::Pressed {
            let (camera, camera_transform) = *camera_query;

            let Ok(window) = windows.get_single() else {
                return;
            };

            let Some(cursor_position) = window.cursor_position() else {
                return;
            };

            let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
                return;
            };

            for (mut sprite, mut cell, _) in query.iter_mut() {
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
                        cell.mark();
                        sprite.color = Color::srgb(0.28, 0.28, 0.28);
                    } else if cell.marked {
                        println!("celula já marcada");
                    } else {
                        println!("não é possivel selecionar essa celula");
                    }
                }
            }
        }
    }
}
