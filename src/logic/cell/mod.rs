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

/**
 * vai conter a logica / regras para o funcionamento da celula do tabuleiro,
 * ela deve conter um atributo para configurar se está marcada ou não, e deve receber tambem
 * um atributo referente a peça do navio que está armazenada nela
 */
impl Cell {

    pub fn mark(
        &mut self, 
        sprite: &mut Sprite,
        ships_query: &Query<(Entity, &Ship)>,
        entity: Entity
    ) {
        let mut validator: bool = false;
        let mut navio_alvo: Option<&Ship> = None;

        //compara referencias
        for (_entity, ship) in ships_query.iter() {
            if ship.cells.contains(&entity) {
                validator = true;
                navio_alvo =  Some(ship);
            }
        }

        if validator {
            println!("A célula clicada pertence ao navio: {:?}", navio_alvo);
            sprite.color = Color::srgb(0.255, 0.0, 0.0,);
        }else{
            println!("atirou no mar");
            sprite.color = Color::srgb(0.28, 0.28, 0.28);
        }
        
        self.marked = true;
    }
}
