use bevy::prelude::*;
use super::animation::AnimationTimer;

#[derive(Component)]
pub struct JoueurPrincipal;

#[derive(Bundle)]
pub struct BundleJoueur {
    pub joueur: JoueurPrincipal,

    pub velocity: Velocity,

    pub sprite: SpriteBundle,

    pub texture: TextureAtlas,

    pub animation_timer: AnimationTimer,

    pub name: Name
}

impl Default for BundleJoueur {
    fn default() -> Self {
        BundleJoueur {
            // Permet d'identifier l'entité
            joueur: JoueurPrincipal,
            name: Name::new("joueur"),

            // On lui donne une velocité,
            // sa position pourra alors être mise à jours.
            velocity: Velocity::default(),

            sprite: SpriteBundle::default(),

            // L'utilitaire qui gères l'image de l'animation affichée
            texture: TextureAtlas::default(),

            // On ajoute un composant contenant l'état
            // de l'animation (basiquement le temps entre chaque image).
            animation_timer: AnimationTimer(
                Timer::from_seconds(0.1, TimerMode::Repeating)
            )
        }
    }
}

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