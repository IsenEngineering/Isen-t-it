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

    // Les images / textures de la scene
    let scene = asset_server.load(SCENE_FILE_PATH);
    let sol = asset_server.load(SOL_FILE_PATH);

    commands.spawn((
        SpriteBundle {
            texture: scene,
            transform: Transform {
                /* Scene de 480x336, origine à 0, 0
                en mettant la scene aux coords: 240, 168;

                les coordonnées 0, 0 correspondent au bas gauche de la scene 
                +24px en y pour que mettre le sol */
                translation: Vec3::new(240.0, 168.0 + 24.0, 0.),
                ..default()
            },
            ..default()
        },
    ));

    commands.spawn((
        SpriteBundle {
            texture: sol,
            transform: Transform {
                // Sol de 504x24, origine à 0, 0
                translation: Vec3::new(226., 12., 0.),
                ..default()
            },
            ..default()
        },
    ));
}