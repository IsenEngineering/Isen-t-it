use bevy::prelude::*;

// Velocité d'une entité
#[derive(Clone, Component)]
pub struct Velocity {
	pub dx: f32,
	pub dy: f32,
}