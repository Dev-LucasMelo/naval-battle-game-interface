mod logic;
mod ui;

use bevy::prelude::*;
use ui::components::{
    board::Board,
    ship_selection_panel::ShipSelectionPanel,
    ships::{check_sunk_change, debug_spawn_submarine},
};

fn main() {
    env_logger::init();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Board) // adicionando plugin de tabuleiro
        .add_plugins(ShipSelectionPanel)
        .add_systems(Startup, setup)
        .add_systems(PostStartup, debug_spawn_submarine)
        .add_systems(Update, check_sunk_change)
        .run();
}

//função setup serve para inicializar o sistema e configurações gerais do ambiente gráfico
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
