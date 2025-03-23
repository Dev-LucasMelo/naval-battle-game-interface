mod ui;
mod logic;

use bevy::prelude::*;
use ui::components::ships::debug_spawn_submarine;

fn main() {
    env_logger::init();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ui::components::board::Board) // adicionando plugin de tabuleiro
        .add_systems(Startup, setup)
        .add_systems(PostStartup, debug_spawn_submarine)
        .run();
}

//função setup serve para inicializar o sistema e configurações gerais do ambiente gráfico
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d); //cria uma cena 2d assim que inicializar o programa
}
