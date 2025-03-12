use crate::logica_do_jogo::celula::Celula;
use bevy::input::{mouse::MouseButtonInput, ButtonState};

pub use bevy::prelude::*;

pub struct Structtabuleiro;

/**
 * criando um plugin bevy para criar o
 * tabuleiro a partir de uma função que vai ser chamada
 * a partir da inicialização do bevy
 */
impl Plugin for Structtabuleiro {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, renderizar_tabuleiro);
        app.add_systems(Update, detectar_click);
    }
}

//configs globais do tabuleiro
const TAMANHO_BLOCO: f32 = 50.0;
const ESPACAMENTO: f32 = 4.0;

const LINHAS: usize = 10;
const COLUNAS: usize = 10;

fn renderizar_tabuleiro(mut commands: Commands) {
    for linha in 0..LINHAS {
        for coluna in 0..COLUNAS {
            let cor = if linha < LINHAS / 2 {
                //parte do jogador atual
                Color::srgb(0.4, 0.7, 1.0)
            } else {
                //parte do inimigo
                Color::srgb(0.0, 0.2, 0.4)
            };

            let posicao_x = (coluna as f32) * (TAMANHO_BLOCO + ESPACAMENTO)
                - (COLUNAS as f32 * (TAMANHO_BLOCO + ESPACAMENTO) / 2.0);
            let posicao_y = (linha as f32) * (TAMANHO_BLOCO + ESPACAMENTO)
                - (LINHAS as f32 * (TAMANHO_BLOCO + ESPACAMENTO) / 2.0);

            // Configuracoes do bevy para setar cor tamanho e posicionamento (aplicação de translação)
            commands
                .spawn(Sprite {
                    color: cor,
                    custom_size: Some(Vec2::new(TAMANHO_BLOCO, TAMANHO_BLOCO)), //cria quadrados
                    ..Default::default()
                })
                .insert(
                    Transform {
                        translation: Vec3::new(posicao_x, posicao_y, 0.0), // Posição centralizada
                        ..Default::default()
                    }, // posicao do tabuleiro
                )
                .insert(Celula {
                    coluna,
                    linha,
                    marcada: false,
                });
        }
    }
}

fn detectar_click(
    mut query: Query<(&mut Sprite, &mut Celula, &Transform)>,
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

            for (mut sprite, mut celula, _) in query.iter_mut() {
                let coluna = celula.coluna;
                let linha = celula.linha;

                // Cálculo da posição da célula no tabuleiro
                let posicao_x = (coluna as f32) * (TAMANHO_BLOCO + ESPACAMENTO)
                    - (COLUNAS as f32 * (TAMANHO_BLOCO + ESPACAMENTO) / 2.0);
                let posicao_y = (linha as f32) * (TAMANHO_BLOCO + ESPACAMENTO)
                    - (LINHAS as f32 * (TAMANHO_BLOCO + ESPACAMENTO) / 2.0);

                // area da celula (precisa refinar)
                let area_celula = Rect {
                    min: Vec2::new(posicao_x - TAMANHO_BLOCO / 2.0, posicao_y - TAMANHO_BLOCO / 2.0),
                    max: Vec2::new(posicao_x + TAMANHO_BLOCO / 2.0, posicao_y + TAMANHO_BLOCO / 2.0),
                };

                if area_celula.contains(point.xy()) {
                    
                    if celula.linha > (LINHAS / 2) - 1 && !celula.esta_marcada() { 
                        celula.marcar();
                        sprite.color = Color::srgb(0.28, 0.28, 0.28);
                    }else if celula.esta_marcada(){
                        println!("celula já marcada");
                    }
                    else{
                        println!("não é possivel selecionar essa celula");
                    }
                }
            }
        }
    }
}
