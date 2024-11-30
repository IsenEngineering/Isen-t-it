use bevy::prelude::*;
use composants::{JoueurPrincipal, Velocity};

mod animation;
mod mouvements;
pub mod composants;

/*   Constantes  */

// Fichier du sprite (le personnage avec ses animations)
const PLAYER_SPRITESHEET: &str = "dino/mort.png";
// Vitesse  d'un personnage
pub const PLAYER_SPEED: f32 = 50.0;
// Vitesse  d'un personnage en sprint
pub const PLAYER_SPRINT_SPEED: f32 = 100.0;

pub struct Joueur;
impl Plugin for Joueur {
    fn build(&self, app: &mut App) {
        // Au démarrage
        app.add_systems(Startup, setup);

        // à chaque image
        app.add_systems(
            Update,
            (
                mouvements::move_sprite,
                mouvements::move_sprite_touches.after(mouvements::move_sprite),
                animation::animate_sprite,
            ),
        );
    }
}

/* Systèmes */

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Les animations du personnage
    let texture = asset_server.load(PLAYER_SPRITESHEET);
    // Les images de l'animation sur une grille
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 24, 1, None, None);

    // Un utilitaire qui permet de manipuler
    // quelle image de l'animation on affiche
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // On spawn l'entité
    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            }),
            ..default()
        },
        Transform {
            // z: 2 pour que le joueur soit au dessus du fond

            // On la dépose au centre
            translation: Vec3::new(24., 24., 2.),
            scale: Vec3::new(1., 1., 1.),
            ..default()
        },
        JoueurPrincipal,
        Velocity::default(),
        animation::AnimationTimer(
            Timer::from_seconds(0.1, TimerMode::Repeating)
        ),
    ));
}
