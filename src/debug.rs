use bevy::{
    prelude::*,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};
use dotenv::dotenv;
use bevy_editor_pls::EditorPlugin;
use std::env;

#[derive(Resource)]
pub struct Debug(bool);

pub struct PluginPerf;

impl Plugin for PluginPerf {
    fn build(&self, app: &mut App) {
        // On charge les variables d'environnement contenus dans le fichier .env
        // il peut notamnent contenir `DEBUG="true"`
        dotenv().ok();

        let debug = match env::var("DEBUG") {
            Ok(v) => v,
            _ => "false".to_string()
        };

        app.insert_resource(Debug(debug == "true"));
        // On mets à disposition les outils de développement
        // si l'utilisateur est en mode debug.
        if debug == "true" {
            app.add_plugins((
                EditorPlugin::new(),
                FrameTimeDiagnosticsPlugin,
                LogDiagnosticsPlugin::default()
            ));
        }

    }
}

pub fn is_debug(debug: Res<Debug>) -> bool {
    debug.0
}