use bevy::prelude::*;
use crate::composants::Velocity;

/*   Constantes  */

// Fichier du sprite (le personnage avec ses animations)
const PLAYER_SPRITESHEET: &str = "dino/mort.png";
// Vitesse  d'un personnage
const PLAYER_SPEED: f32 = 50.0;
// Vitesse  d'un personnage en sprint
const PLAYER_SPRINT_SPEED: f32 = 100.0;

/* Plugin */
    
pub struct PluginJoueur;

impl Plugin for PluginJoueur {
	fn build(&self, app: &mut App) {
        // Au démarrage
        app.add_systems(Startup, setup);

        // à chaque image
        app.add_systems(Update, (move_sprite, animate_sprite));
    }
}

/* Systèmes */

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, 
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {

    // Les animations du personnage
    let texture = asset_server.load(PLAYER_SPRITESHEET);
    // Les images de l'animation sur une grille
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(24), 24, 1, None, None);

    // Un utilitaire qui permet de manipuler 
    // quelle image de l'animation on affiche
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // On spawn l'entité
    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform {
                // z: 2 pour que le joueur soit au dessus du fond

                // On la dépose au centre
                translation: Vec3::new(24., 24., 2.),
                scale: Vec3::new(1., 1., 1.),
                ..default()
            },
            ..default()
        },
        // On lui donne une velocité, 
        // sa position est alors mise à jours.
        Velocity {
            dx: 0.0,
            dy: 0.0
        },

        // L'utilitaire qui gères l'image de l'animation affichée
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        },

        // On ajoute un composant contenant l'état 
        // de l'animation (basiquement le temps entre chaque image).
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
    ));
}

fn move_sprite(keyboard: Res<ButtonInput<KeyCode>>,
    mut sprite_position: Query<&mut Velocity, With<Sprite>>) {
    // Il n'y a par défaut qu'un unique sprite.
    let mut v = sprite_position.single_mut();

    // Norme du vecteur de velocité
    let norme: f32 = match keyboard.pressed(KeyCode::ShiftLeft) {
        // appuyer sur shift permet de courir plus vite
        true => PLAYER_SPRINT_SPEED,
        false => PLAYER_SPEED
    };

    // On donne un mouvement sur l'axe Y
    v.dy = 0.0;
    if keyboard.pressed(KeyCode::KeyW) {
        v.dy += norme;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        v.dy -= norme;
    } 
    
    // On donne un mouvement sur l'axe X
    v.dx = 0.0;
    if keyboard.pressed(KeyCode::KeyA) {
        v.dx -= norme;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        v.dx += norme;
    }
}

// Composant contenant l'état de l'animation
#[derive(Component)]
struct AnimationTimer(Timer);

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

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &Velocity,
        &mut AnimationTimer,
        &mut TextureAtlas,
        &mut Sprite
    )>,
) {
    for (velocity, mut timer, mut texture, mut sprite) in query.iter_mut() {
        // On mets à jour l'état de l'animation
        timer.0.tick(time.delta());

        // On vérifie si l'entité est en mouvement.
        let is_moving = velocity.dx != 0.0 || velocity.dy != 0.0;
        let is_sprinting = velocity.dx.abs() >= PLAYER_SPRINT_SPEED || 
            velocity.dy.abs() >= PLAYER_SPRINT_SPEED;
        
        // si l'entité est mouvement.
        if velocity.dx != 0.0 {
            // On ajuste le côté où elle regarde.
            sprite.flip_x = velocity.dx < 0.0;

            // Notez que flip_x est un boolean, 
            // on assigne bien une condition <=> un boolean
        }

        // Dans le cas où l'entité bouge, on l'anime.
        if is_moving && timer.0.finished() {
            texture.index = match is_sprinting {
                true => update_animation(texture.index, 17, 23),
                false => update_animation(texture.index, 4, 9)
            }
        } else if !is_moving {
            // L'image par défaut.
            texture.index = 0;
        }
    }
}