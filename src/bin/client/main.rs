use bevy::{color::palettes::css::BLACK, prelude::*};

use isent_it::{
    collisions,
    debug,
    joueur,
    lumieres,
    monde,
    systems
};

mod network;

fn main() {
    // Application Bevy
    let mut app = App::new();

    // DefaultPlugins ajoute les briques de bases
    // pour la création rapide d'un jeu. (moteur 2D, fenetre...)
    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin {
                // Ce "sampler" donne un look pixelisé au jeu,
                // Vous pouvez essayer dd supprimer cette ligne,
                // toutes les images deviennent floues
                default_sampler: bevy::image::ImageSamplerDescriptor::nearest(),
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    // On dit où mettre le jeu lorsqu'on
                    // compile pour le web (site internet / wasm / WebAssembly)
                    canvas: Some("#bevy-canvas".to_string()),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
        joueur::Joueur,
        monde::Monde,
        debug::PluginPerf,
        collisions::PluginCollisions,
        lumieres::PluginLumieres,
        network::Client,
    ));

    // Au démarrage
    app.add_systems(Startup, setup);

    // à chaque image
    app.add_systems(
        Update,
        (
            systems::movement_system,
            systems::on_resize_system,
            systems::camera_follow_system,
        ),
    );
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        // // On mets une camera, autrement on pourrait pas voir ce qu'il se passe...
        Camera2d, 
        Camera {
            // // La technologie HDR permet d'améliorer les lumières
            // // On va "saturer" les couleurs (en bref)
            hdr: true,
            // // Fond blanc
            clear_color: ClearColorConfig::Custom(BLACK.into()),
            ..default()
        },
        // // On centre l'axe y de la camera sur le premier étage
        Transform::from_translation(Vec3::new(0.0, 36.0, 0.0))
    ));
}
