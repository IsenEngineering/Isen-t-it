use bevy::{prelude::*, window::WindowResized};
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

pub fn camera_follow_system(
    time: Res<Time>,
    player_query: Query<&Transform, With<Velocity>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Velocity>)>,
) {
    let player = match player_query.get_single() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    let mut cam = match camera_query.get_single_mut() {
        Ok(transform) => transform,
        Err(_) => return,
    };
    let delta: Vec3 = player.translation - cam.translation;
	cam.translation.x += delta.x * time.delta_seconds() * 4.0;
}

pub fn on_resize_system(
    mut q: Query<&mut OrthographicProjection, With<Camera>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for e in resize_reader.read() {
        for mut projection in q.iter_mut() {
            projection.scale = 48.0 / e.height;
        }
    }
}