use bevy::prelude::*;

#[derive(Component)]
pub struct JoueurPrincipal;

// Velocité d'une entité
#[derive(Component)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Velocity {
            dx: 0.0,
            dy: 0.0
        }
    }
}