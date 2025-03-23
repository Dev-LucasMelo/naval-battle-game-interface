use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Cell {
    pub row: usize,
    pub column: usize,
    pub marked: bool
}

/**
 * vai conter a logica / regras para o funcionamento da celula do tabuleiro,
 * ela deve conter um atributo para configurar se está marcada ou não, e deve receber tambem
 * um atributo referente a peça do navio que está armazenada nela
 */
impl Cell {
    pub fn mark(&mut self) {
        self.marked = true;
    }
}
