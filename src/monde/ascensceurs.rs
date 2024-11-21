use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

// Chemin de l'image de l'ascenseur (qui fait 36x42)
const ASCENSEUR_PATH: &str = "monde/ascenseur.png";

pub struct PluginAscenseur;

impl Plugin for PluginAscenseur {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>) {

    // On charge l'image de l'ascenseur
    let ascenseur = asset_server.load(ASCENSEUR_PATH);

    // Pour chaque étage
    for i in 0..7 {
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            material: materials.add(ColorMaterial {
                texture: Some(ascenseur.clone()),
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(
                    // 6 ème carré de la largeur et au milieu
                    6.0 * 48.0 + 24.0, 
                    45.0 + i as f32 * 48.0, 
                    1.0
                ),
                scale: Vec3::new(36.0, 42.0, 1.0),
                ..default()
            },
            ..default()
        }).insert(Name::from(format!("ascenseur-{}", i + 1)));
    }
}