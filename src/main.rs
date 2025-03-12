mod ui;
mod logica_do_jogo;

use bevy::prelude::*;

fn main() {
    //logs
    env_logger::init();

    // adicionar plugins gráficos de cada componente separadamente
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ui::componentes::tabuleiro::Structtabuleiro) // adicionando plugin de tabuleiro
        .add_systems(Startup, setup)
        .run();
}

//função setup serve para inicializar o sistema e configurações gerais do ambiente gráfico
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d); //cria uma cena 2d assim que inicializar o programa
}
