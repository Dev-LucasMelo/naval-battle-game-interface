mod ui;

use bevy::prelude::*;
use ui::componentes::teste::TextoComponent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _ = asset_server;
    
    commands.spawn(Camera2d);
  
    commands.spawn((
        Text2d::new("componente 1"),
        Transform::from_translation(Vec3::new(-100.0, 100.0, 0.0)),
    ));

    let componente = TextoComponent::new("componente 2");

    componente.criar_texto_2d(&mut commands);
}
