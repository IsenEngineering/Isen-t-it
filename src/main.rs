use bevy::{
    color::palettes::css::WHITE,
    prelude::*
};

mod composants;
mod joueur;
mod scene;
mod systems;
mod performance;

fn main() {
    let mut app = App::new();
    
    // DefaultPlugins ajoute les briques de bases 
    // pour la création rapide d'un jeu.
    app.add_plugins((
        DefaultPlugins.set(ImagePlugin {
            default_sampler: bevy::render::texture::ImageSamplerDescriptor::nearest()
        }),
        joueur::PluginJoueur,
        scene::PluginScene,
        performance::PluginFPS
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
    let mut camera: Camera2dBundle = Camera2dBundle::default();

    camera.camera.clear_color = ClearColorConfig::Custom(WHITE.into());
    camera.transform.translation.y = 24.0;
    
    commands.spawn(camera);

}