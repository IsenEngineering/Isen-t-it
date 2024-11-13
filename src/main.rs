use bevy::{
    color::palettes::css::WHITE,
    prelude::*
};

mod composants;
mod joueur;
mod systems;

fn main() {
    let mut app = App::new();
    
    // DefaultPlugins ajoute les briques de bases 
    // pour la création rapide d'un jeu.
    app.add_plugins((
        DefaultPlugins.set(ImagePlugin {
            default_sampler: bevy::render::texture::ImageSamplerDescriptor::nearest()
        }),
        joueur::PluginJoueur
    ));

    // Au démarrage
    app.add_systems(Startup, setup);

    // à chaque image
    app.add_systems(Update, systems::movement_system);
    app.run();
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.camera.clear_color = ClearColorConfig::Custom(WHITE.into());
    camera.projection.scale = 0.5;
    
    commands.spawn(camera);
}