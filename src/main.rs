use bevy::prelude::*;

mod joueur;
mod systems;
mod debug;
mod collisions;
mod lumieres;
mod monde;

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
            default_sampler: bevy::render::texture::ImageSamplerDescriptor::nearest()
        })
        .set(WindowPlugin {
            primary_window: Some(Window{
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
        lumieres::PluginLumieres
    ));

    // à chaque image
    app.add_systems(Update, (
        systems::movement_system, 
        systems::on_resize_system,
        systems::camera_follow_system
    ));
    app.run();
}