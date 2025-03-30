use bevy::prelude::*;

use crate::ui::components::ships::Ship;

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
    ) {
        clicked_cells.push(entity_alvo);

        let mut validator: bool = false;
        
        for (_entity,mut ship) in ships_query.iter_mut() {
        
            if ship.cells.contains(&entity_alvo) {
                validator = true;
                let all_cells_clicked = ship.cells.iter().all(|cell| clicked_cells.contains(cell));

                if all_cells_clicked {
                    ship.sunk = true;
                }

                break;
            }
        
        }

        if validator {
            sprite.color = Color::srgb(0.255, 0.0, 0.0);
        } else {
            sprite.color = Color::srgb(0.28, 0.28, 0.28);
        }

        self.marked = true;
    }
}
