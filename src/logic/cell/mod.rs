use bevy::prelude::*;

use crate::ui::components::{board::GameState, ships::Ship};

#[derive(Component, Debug)]
pub struct Cell {
    pub row: usize,
    pub column: usize,
    pub marked: bool,
}

#[derive(Component, Debug, PartialEq)]
pub enum CellSide {
    Player,
    Enemy,
}

impl Cell {
    pub fn mark(
        &mut self,
        sprite: &mut Sprite,
        ships_query: &mut Query<(Entity, &mut Ship)>,
        entity_alvo: Entity,
        clicked_cells: &mut Vec<Entity>, //referencia do vetor de celulas clicadas
        game_state: &mut ResMut<GameState>,
        
    ) {
        clicked_cells.push(entity_alvo);

        let mut validator: bool = false;

        for (_entity, mut ship) in ships_query.iter_mut() {
            if ship.cells.contains(&entity_alvo) {
                validator = true;
                let all_cells_clicked = ship.cells.iter().all(|cell| clicked_cells.contains(cell));

                if all_cells_clicked && game_state.is_player_turn {
                    game_state.player_score += 1;
                    ship.sunk = true;
                } else if all_cells_clicked && !game_state.is_player_turn {
                    game_state.bot_score += 1;
                    ship.sunk = true;
                }

                break;
            }
        }

        if validator && game_state.is_player_turn {
            //se o jogador acertar
            sprite.color = Color::srgb(0.0, 1.0, 0.0);
        } else if validator && !game_state.is_player_turn {
            // se o bot acertar
            sprite.color = Color::srgb(1.0, 0.0, 0.0);
        } else {
            sprite.color = Color::srgb(0.28, 0.28, 0.28);
        }

        //validar fim de game
        if game_state.total_ships_bot == game_state.player_score {
            //jogador ganha
            game_state.winner = Some(0);
        } else if game_state.total_ships_player == game_state.bot_score {
            //bot ganha
            game_state.winner = Some(1);
        }

        game_state.is_player_turn = !game_state.is_player_turn; //alternar jogada
        self.marked = true;
    }
}
