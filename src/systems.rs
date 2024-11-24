use bevy::{prelude::*, window::WindowResized};
use crate::joueur::Velocity;
use crate::collisions::{point_in_area, CollisionArea};

/*
    Ce système doit appliquer la velocité si et uniquement si, 
    cette application n'est pas hors de la zone de collision.

    Autrement dit, il faut que le mouvement reste à l'intérieur de la zone jouable...
    Zone jouable définit par les Composants CollisionArea
*/
pub fn movement_system(
	time: Res<Time>,
    collisions: Query<&CollisionArea>,
	mut query: Query<(&mut Transform, &Velocity), Changed<Velocity>>,
) {
    // On parcourt toutes entités ayant le composant Transform & Velocity
	for (mut transform, velocity) in query.iter_mut() {

        // Le vecteur de translation de l'entité
		let translation: &mut Vec3 = &mut transform.translation;

        // Logique de la vérification des zones de collisions
        let mut outside = false;
        // On parcourt toutes les zones de collisions
        for area in collisions.iter() {
            // On vérifie si la nouvelle position de 
            // l'entitée n'est pas hors de la zone jouable
            if !point_in_area(Vec2::from([
                translation.x + velocity.dx * time.delta_seconds(),
                translation.y + velocity.dy * time.delta_seconds()
            ]), area) {
                outside = true
            }
        }
        if outside {
            // Si l'entité est hors de la zone
            // On n'applique pas la velocite, 
            // On passe à l'entité suivante
            continue;
        }
        
        /* time.delta_seconds() donne le temps entre l'image 
        précedente et la nouvelle, 

        utiliser * time.delta_seconds() assure que le mouvement 
        ne diffère pas d'un appareil performant (beaucoup d'images 
        par seconde) à un appareil moins performant */
		translation.x += velocity.dx * time.delta_seconds();
		translation.y += velocity.dy * time.delta_seconds();
	}
}

/* Ce système ajuste la position de la caméra
(l'écran) sur l'axe x en fonction de la position du joueur

On fait en sorte qu'il y ait un petit delai,
pour que les mouvements de caméra ne soient pas
trop brusques.
*/
pub fn camera_follow_system(
    time: Res<Time>,
    player_query: Query<&Transform, With<Velocity>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Velocity>)>,
) {
    let player = match player_query.get_single() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    for mut cam in camera_query.iter_mut() {
        let delta: Vec3 = player.translation - cam.translation;
    
        // * time.delta_seconds() assure que la camera suit au 
        // fur et à mesure le joueur et pas de façon immédiate
        // * 4.0 assure que la caméra va plus vite que le personnage
        cam.translation.x += delta.x * time.delta_seconds() * 4.0;
    }
}

/* Ce système ajuste la projection de la caméra en fonction de sa taille.
On veut que la hauteur de l'écran affiche 72px (48px de murs + 24px de sols)
La largeur n'importe pas.

*/
pub fn on_resize_system(
    mut q: Query<&mut OrthographicProjection, With<Camera>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for e in resize_reader.read() {
        for mut projection in q.iter_mut() {
            projection.scale = 72.0 / e.height;
        }
    }
}