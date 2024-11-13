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
        DefaultPlugins,
        joueur::PluginJoueur
    ));

    // Au démarrage
    app.add_systems(Startup, setup);

    // à chaque image
    app.add_systems(Update, systems::movement_system);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            // Couleur du fond de l'écran
            clear_color: ClearColorConfig::Custom(WHITE.into()),
            ..default()
        },
        ..default()
    });
}