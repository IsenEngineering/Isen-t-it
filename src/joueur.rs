use bevy::prelude::*;
use crate::composants::Velocity;

// Constantes

const PLAYER_SPRITESHEET: &str = "dino/doux.png";
const PLAYER_SPEED: f32 = 50.0;

// Plugin/Setup:
    
pub struct PluginJoueur;

impl Plugin for PluginJoueur {
	fn build(&self, app: &mut App) {
        // Au démarrage
        app.add_systems(Startup, setup);

        // à chaque image
        app.add_systems(Update, move_sprite);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, 
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    let texture = asset_server.load(PLAYER_SPRITESHEET);
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(24), 24, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform {
                translation: Vec3::new(100., 0., 0.),
                scale: Vec3::new(2., 2., 1.),
                ..default()
            },
            ..default()
        },
        Velocity {
            dx: 0.0,
            dy: 0.0
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        }
    ));
}


fn move_sprite(keyboard: Res<ButtonInput<KeyCode>>,
    mut sprite_position: Query<&mut Velocity, With<Sprite>>) {
    // La vélocité du sprite
    let mut v = sprite_position.single_mut();

    // Mouvements sur l'axe Y
    v.dy = 0.0;
    if keyboard.pressed(KeyCode::KeyW) {
        v.dy += PLAYER_SPEED;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        v.dy -= PLAYER_SPEED;
    } 
    
    // Mouvements sur l'axe X
    v.dx = 0.0;
    if keyboard.pressed(KeyCode::KeyA) {
        v.dx -= PLAYER_SPEED;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        v.dx += PLAYER_SPEED;
    }
}