use bevy::prelude::*;
use crate::network::Player;

pub mod animation;
mod mouvements;
pub mod composants;

/*   Constantes  */

// Fichier du sprite (le personnage avec ses animations)
const PLAYER_SPRITESHEET: &str = "dino/mort.png";
const PLAYER_SPRITESHEET_DOUX: &str = "dino/doux.png";
const PLAYER_SPRITESHEET_TARD: &str = "dino/tard.png";
const PLAYER_SPRITESHEET_VITA: &str = "dino/vita.png";
// Vitesse  d'un personnage
pub const PLAYER_SPEED: f32 = 50.0;
// Vitesse  d'un personnage en sprint
pub const PLAYER_SPRINT_SPEED: f32 = 100.0;

pub struct PluginJoueur;
impl Plugin for PluginJoueur {
    fn build(&self, app: &mut App) {
        // Au démarrage
        // app.add_systems(Startup, setup);

        // à chaque image
        app.add_systems(
            PreUpdate, 
            (
            mouvements::move_sprite,
            mouvements::move_sprite_touches,
            ).chain()
        );
        app.add_systems(
            PostUpdate,
            (
                animation::animate_local_sprite,
                animation::animate_players
            ),
        );
    }
}

pub fn spawn_player(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    player: Player
) -> Entity {
    // Les animations du personnage
    let texture = match player.skin % 4 {
        0 => asset_server.load(PLAYER_SPRITESHEET),
        1 => asset_server.load(PLAYER_SPRITESHEET_DOUX),
        2 => asset_server.load(PLAYER_SPRITESHEET_TARD),
        _ => asset_server.load(PLAYER_SPRITESHEET_VITA),
    };

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
            translation: player.position,
            scale: Vec3::new(1., 1., 1.),
            ..default()
        },
        animation::AnimationTimer(
            Timer::from_seconds(0.1, TimerMode::Repeating)
        ),
        player
    )).id()
}