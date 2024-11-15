use bevy::{
    color::palettes::css::WHITE,
    prelude::*
};

mod composants;
mod joueur;
mod scene;
mod systems;
mod performance;
mod collisions;

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
        performance::PluginFPS,
        collisions::PluginCollisions
    ));

    // Au démarrage
    app.add_systems(Startup, setup);

    // à chaque image
    app.add_systems(Update, (
        systems::movement_system, 
        systems::on_resize_system,
        systems::camera_follow_system
    ));
    app.run();
}

fn setup(mut commands: Commands) {
    // On mets une camera, autrement on pourrait pas voir ce qu'il se passe...
    let mut camera: Camera2dBundle = Camera2dBundle::default();

    // Fond blanc
    camera.camera.clear_color = ClearColorConfig::Custom(WHITE.into());
    
    // On centre l'axe y de la camera sur le premier étage
    camera.transform.translation.y = 36.0;
    
    commands.spawn(camera);
}