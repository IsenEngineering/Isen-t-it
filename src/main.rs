use bevy::{
    color::palettes::css::WHITE, core_pipeline::tonemapping::Tonemapping, prelude::*
};
use dotenv::dotenv;

mod joueur;
mod systems;
mod debug;
mod collisions;
mod lumieres;
mod monde;

fn main() {
    // On charge les variables d'environnement contenus dans le fichier .env
    // il peut notamnent contenir `DEBUG="true"`
    dotenv().ok();

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
            primary_window: Some( Window{
                // On dit où mettre le jeu lorsqu'on
                // compile pour le web (site internet / wasm / WebAssembly)
                canvas: Some("#bevy-canvas".to_string()),
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

    // La technologie HDR permet d'améliorer les lumières
    // On va "saturer" les couleurs (en bref)
    camera.camera.hdr = true;
    // jsp ce que c'est ça
    camera.tonemapping = Tonemapping::TonyMcMapface;

    // Fond blanc
    camera.camera.clear_color = ClearColorConfig::Custom(WHITE.into());
    
    // On centre l'axe y de la camera sur le premier étage
    camera.transform.translation.y = 36.0;
    
    commands.spawn(camera);
}