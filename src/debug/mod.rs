use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
// use bevy_editor_pls::EditorPlugin;
use dotenv::dotenv;

mod information;

#[derive(Resource)]
pub struct Debug(bool);

pub struct PluginPerf;

impl Plugin for PluginPerf {
    fn build(&self, app: &mut App) {
        // On charge les variables d'environnement contenus dans le fichier .env
        // il peut notamnent contenir `DEBUG="true"`
        dotenv().ok();

        let debug = match std::env::var("DEBUG") {
            Ok(v) => v,
            _ => "false".to_string(),
        };

        app.insert_resource(Debug(debug == "true"));
        // On mets à disposition les outils de développement
        // si l'utilisateur est en mode debug.
        app.add_systems(Startup, information::setup);
        app.add_systems(Update, information::update);
        if debug == "true" {
            app.add_plugins(FrameTimeDiagnosticsPlugin);
            // Le plugin bevy_pls_editor n'est pas compatible avec la 0.15 de bevy
            // EditorPlugin::new() 
        }
    }
}

pub fn is_debug(debug: Res<Debug>) -> bool {
    debug.0
}
