use bevy::prelude::*;
use crate::composants::Velocity;

// Mets en mouvements toutes les entités 
// ayant le composant "transform" et une velocité.
pub fn movement_system(
	time: Res<Time>,
	mut query: Query<(&mut Transform, &Velocity)>,
) {
	for (mut transform, velocity) in query.iter_mut() {
		let translation: &mut Vec3 = &mut transform.translation;
		translation.x += velocity.dx * time.delta_seconds();
		translation.y += velocity.dy * time.delta_seconds();
	}
}