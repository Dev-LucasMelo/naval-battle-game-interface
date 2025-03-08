pub use bevy::prelude::*;
pub struct TextoComponent {
    valor: String,
}

impl TextoComponent {
    pub fn new(valor: &str) -> Self {
        TextoComponent {
            valor: valor.to_string(),
        }
    }

    pub fn criar_texto_2d(
        &self,
        commands: &mut Commands,
    ) {
        commands.spawn((
            Text2d::new(&self.valor),
            Transform::from_translation(Vec3::new(0.0, 50.0, 0.0)),
            
        ));
    }
}
