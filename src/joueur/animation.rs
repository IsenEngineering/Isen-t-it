use crate::{joueur::{composants::Velocity, PLAYER_SPRINT_SPEED}, network::Player};
use bevy::prelude::*;

use super::composants::JoueurPrincipal;

// Composant contenant l'état de l'animation
#[derive(Component)]
pub struct AnimationTimer(pub Timer);

// Trouve l'indice de l'image suivante de l'animation
fn update_animation(indice: usize, min: usize, max: usize) -> usize {
    if indice < min || indice > max {
        // Première image de l'animation
        min
    } else {
        // On passe sur chaque image de l'animation de course.
        min + (indice + 1 - min) % (max - min)
    }
}

pub fn animate_local_sprite(
    time: Res<Time>,
    mut query: Query<
        (
            &Velocity,
            &mut AnimationTimer,
            &mut Sprite,
        ),
        (Changed<Velocity>, With<JoueurPrincipal>),
    >,
) {
    for (velocity, mut timer, mut sprite) in query.iter_mut() {
        // On mets à jour l'état de l'animation
        timer.0.tick(time.delta());

        // On vérifie si l'entité est en mouvement.
        let is_moving = velocity.dx != 0.0 || velocity.dy != 0.0;
        let is_sprinting =
            velocity.dx.abs() >= PLAYER_SPRINT_SPEED || velocity.dy.abs() >= PLAYER_SPRINT_SPEED;

        // si l'entité est mouvement.
        if velocity.dx != 0.0 {
            // On ajuste le côté où elle regarde.
            sprite.flip_x = velocity.dx < 0.0;

            // Notez que flip_x est un boolean,
            // on assigne bien une condition <=> un boolean
        }

        let texture = match &mut sprite.texture_atlas {
            Some(v) => v,
            _ => return
        };

        // Dans le cas où l'entité bouge, on l'anime.
        if is_moving && timer.0.finished() {
            texture.index = match is_sprinting {
                true => update_animation(texture.index, 17, 23),
                false => update_animation(texture.index, 4, 9),
            }
        } else if !is_moving {
            // L'image par défaut.
            texture.index = 0;
        }
    }
}

pub fn animate_players(
    time: Res<Time>,
    mut query: Query<
        (
            &mut Player,
            &mut AnimationTimer,
            &mut Sprite,
        ),
        Without<JoueurPrincipal>,
    >,
) {
    for (mut player, mut timer, mut sprite) in query.iter_mut() {
        // On mets à jour l'état de l'animation
        timer.0.tick(time.delta());

        // On vérifie si l'entité est en mouvement.
        let is_moving = player.vitesse > 0.0;
        let is_sprinting = player.vitesse >= PLAYER_SPRINT_SPEED * 0.75;

        if player.vitesse > 0.0 {
            player.vitesse *= time.delta_secs();
        }
        
        // si l'entité est mouvement.
        if is_moving {
            // On ajuste le côté où elle regarde.
            sprite.flip_x = player.angle.abs() >= 90;

            // Notez que flip_x est un boolean,
            // on assigne bien une condition <=> un boolean
        }

        let texture = match &mut sprite.texture_atlas {
            Some(v) => v,
            _ => return
        };

        // Dans le cas où l'entité bouge, on l'anime.
        if is_moving && timer.0.finished() {
            texture.index = match is_sprinting {
                true => update_animation(texture.index, 17, 23),
                false => update_animation(texture.index, 4, 9),
            }
        } else if !is_moving {
            // L'image par défaut.
            texture.index = 0;
        }
    }
}