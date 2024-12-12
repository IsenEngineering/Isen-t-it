use crate::collisions::{point_in_area, CollisionArea, CollisionDisabled};
use crate::joueur::composants::{Velocity, JoueurPrincipal};
use bevy::{prelude::*, window::WindowResized};

/*
    Ce système doit appliquer la velocité si et uniquement si,
    cette application n'est pas hors de la zone de collision.

    Autrement dit, il faut que le mouvement reste à l'intérieur de la zone jouable...
    Zone jouable définit par les Composants CollisionArea
*/
pub fn movement_system(
    time: Res<Time>,
    collisions: Query<&CollisionArea, Without<CollisionDisabled>>,
    mut query: Query<(&mut Transform, &Velocity), Changed<Velocity>>,
) {
    // On parcourt toutes entités ayant le composant Transform & Velocity
    for (mut transform, velocity) in query.iter_mut() {

        // Logique de la vérification des zones de collisions
        let mut outside = false;
        // On parcourt toutes les zones de collisions
        for area in collisions.iter() {
            // On vérifie si la nouvelle position de
            // l'entitée n'est pas hors de la zone jouable
            if !point_in_area(
                Vec2::from([
                    transform.translation.x + velocity.dx * time.delta_secs(),
                    transform.translation.y + velocity.dy * time.delta_secs(),
                ]),
                area,
            ) {
                outside = true
            }
        }
        if outside || (velocity.dx.abs() < 0.1 && velocity.dy.abs() < 0.1) {
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
        transform.translation.x += velocity.dx * time.delta_secs();
        transform.translation.y += velocity.dy * time.delta_secs();

        // La troisième dimension permet de gérer la profondeur
        // par extension qui doit être afficher 
        // si deux choses se trouvent à la même positon x et y.

        // Par défaut, on mets z à la moitié de l'étage 
        // comme ça z varie entre 0.0 et 24.0, la largeur du couloir.
        // Il faudra changer redéfinir z à chaque changement d'étage.
        transform.translation.z -= velocity.dy * time.delta_secs();
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
    mut set: ParamSet<(
        Query<&Transform, With<JoueurPrincipal>>,
        Query<&mut Transform, With<Camera>>
    )>,
) {
    let player = match set.p0().get_single() {
        Ok(transform) => transform.clone(),
        Err(_) => return,
    };

    for mut cam in set.p1().iter_mut() {
        let delta: Vec3 = player.translation - cam.translation;

        // * time.delta_seconds() assure que la camera suit au
        // fur et à mesure le joueur et pas de façon immédiate
        // * 4.0 assure que la caméra va plus vite que le personnage
        cam.translation.x += delta.x * time.delta_secs() * 4.0;
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