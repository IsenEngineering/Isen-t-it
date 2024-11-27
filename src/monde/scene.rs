use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const SCENE_FILE_PATH: &str = "monde/maquette.png";
const SOL_FILE_PATH: &str = "monde/sol.png";

pub struct PluginScene;

impl Plugin for PluginScene {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Les images / textures de la scene
    let scene = asset_server.load(SCENE_FILE_PATH);
    let sol = asset_server.load(SOL_FILE_PATH);

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        material: materials.add(ColorMaterial {
            texture: Some(scene),
            ..default()
        }),
        transform: Transform {
            /* Scene de 480x336, origine à 0, 0
            en mettant la scene aux coords: 240, 168;

            les coordonnées 0, 0 correspondent au bas gauche de la scene
            +24px en y pour que mettre le sol */
            translation: Vec3::new(240.0, 168.0 + 24.0, 0.),
            scale: Vec3::new(480.0, 336.0, 1.0),
            ..default()
        },
        ..default()
    });

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        material: materials.add(ColorMaterial {
            texture: Some(sol),
            ..default()
        }),
        transform: Transform {
            // Sol de 504x24, origine à 0, 0
            translation: Vec3::new(226., 12., 0.),
            scale: Vec3::new(504.0, 24.0, 1.0),
            ..default()
        },
        ..default()
    });
}
