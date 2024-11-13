use bevy::prelude::*;

const SCENE_FILE_PATH: &str = "maquette.png";

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, 
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {

    // Les animations du personnage
    let texture = asset_server.load(SCENE_FILE_PATH);
    // Les images de l'animation sur une grille
    let layout = TextureAtlasLayout::from_grid(
        UVec2::from_array([480, 48]), 1, 7, None, None);

    // Un utilitaire qui permet de manipuler 
    // quelle image de l'animation on affiche
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // On spawn l'entité
    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform {
                // On la dépose vers le centre
                translation: Vec3::new(100., 0., 0.),
                // On double sa taille
                scale: Vec3::new(2., 2., 1.),
                ..default()
            },
            ..default()
        },

        // L'utilitaire qui gère l'image de l'animation affichée
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 0,
        }
    ));
}