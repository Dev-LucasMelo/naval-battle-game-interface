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
                //parte do inimigo 
                Color::srgb(0.4, 0.7, 1.0)
            } else {
                //parte do jogador atual
                Color::srgb(0.0, 0.2, 0.4)
            };
            
            // Configuracoes do bevy para setar cor tamanho e posicionamento (aplicação de translação)
            commands
                .spawn(Sprite {
                    color: cor,
                    custom_size: Some(Vec2::new(TAMANHO_BLOCO, TAMANHO_BLOCO)), //cria quadrados 
                    ..Default::default()
                })
                .insert(
                    Transform {
                        translation: Vec3::new(
                            coluna as f32 * (TAMANHO_BLOCO + ESPACAMENTO) - (COLUNAS as f32 * (TAMANHO_BLOCO + ESPACAMENTO) / 2.0),
                            linha as f32 * (TAMANHO_BLOCO + ESPACAMENTO) - (LINHAS as f32 * (TAMANHO_BLOCO + ESPACAMENTO) / 2.0),
                            0.0,
                        ),
                        ..Default::default()
                    }, //posicionamento dos blocos no tabuleiro
                );
        }
    }
}
