use bevy::prelude::*;
use composants::{Joueur, JoueurPrincipal, Velocity};
use rand::random;

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
            Update,
            (
                mouvements::move_sprite,
                mouvements::move_sprite_touches.after(mouvements::move_sprite),
                animation::animate_sprite,
            ),
        );
    }
}

pub fn spawn_player(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    skin: u8,
    position: Vec3
) -> Entity {
    // Les animations du personnage
    let texture = match skin % 4 {
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
            translation: position,
            scale: Vec3::new(1., 1., 1.),
            ..default()
        },
        Joueur,
    )).id()
}

/* Systèmes */

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {

    let joueur_principal = spawn_player(
        &mut commands, 
        &asset_server, 
        &mut texture_atlas_layouts, 
        random::<u8>(), 
        Vec3::new(24., 24., 2.)
    );

    commands.entity(joueur_principal).insert((
        JoueurPrincipal,
        Velocity::default(),
        animation::AnimationTimer(
            Timer::from_seconds(0.1, TimerMode::Repeating)
        )
    ));
}
