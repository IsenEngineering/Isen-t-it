use bevy::prelude::*;

const SCENE_FILE_PATH: &str = "maquette.png";
const SOL_FILE_PATH: &str = "sol.png";


pub struct PluginScene;

impl Plugin for PluginScene {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    // Les animations du personnage
    let scene = asset_server.load(SCENE_FILE_PATH);
    let sol = asset_server.load(SOL_FILE_PATH);

    // On spawn l'entité
    commands.spawn((
        SpriteBundle {
            texture: scene,
            transform: Transform {
                // Scene de 480x336, origine à 0, 0
                // Coords du bas gauche : -240, -168
                translation: Vec3::new(240.0, 168.0 + 24.0, 0.),
                scale: Vec3::new(1., 1., 1.),
                ..default()
            },
            ..default()
        },
    ));

    commands.spawn((
        SpriteBundle {
            texture: sol,
            transform: Transform {
                // On la dépose vers le centre
                translation: Vec3::new(226., 12., 0.),
                ..default()
            },
            ..default()
        },
    ));
}