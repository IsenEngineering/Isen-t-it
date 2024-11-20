use bevy::prelude::*;

mod composants;
mod joueur;
mod scene;
mod systems;
mod performance;
mod collisions;
mod lumieres;

fn main() {
    let mut app = App::new();
    
    // DefaultPlugins ajoute les briques de bases 
    // pour la création rapide d'un jeu.
    app.add_plugins((
        DefaultPlugins.set(ImagePlugin {
            // Ce "sampler" donne un look pixelisé au jeu,
            // Vous pouvez essayer en supprimant cette ligne, 
            // toutes les images seraient floues
            default_sampler: bevy::render::texture::ImageSamplerDescriptor::nearest()
        }),
        joueur::PluginJoueur,
        scene::PluginScene,
        performance::PluginPerf,
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