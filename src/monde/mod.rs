use bevy::prelude::*;

mod scene;
mod ascensceurs;

pub struct Monde;

impl Plugin for Monde {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            scene::PluginScene,
            ascensceurs::PluginAscenseur
        ));
    }
}