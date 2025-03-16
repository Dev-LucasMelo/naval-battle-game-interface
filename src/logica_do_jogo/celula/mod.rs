use bevy::prelude::*;

#[derive(Component)]
pub struct Celula {
    pub linha: usize,
    pub coluna: usize,
    pub marcada: bool
}

/**
 * vai conter a logica / regras para o funcionamento da celula do tabuleiro,
 * ela deve conter um atributo para configurar se está marcada ou não, e deve receber tambem
 * um atributo referente a peça do navio que está armazenada nela  
 */
impl Celula {
    pub fn marcar(&mut self) {
        self.marcada = true;
    }

    pub fn esta_marcada(&self) -> bool {
        self.marcada
    }
}