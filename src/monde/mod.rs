use bevy::prelude::*;

mod ascensceurs;
mod scene;

pub struct Monde;

impl Plugin for Monde {
    fn build(&self, app: &mut App) {
        app.add_plugins((scene::PluginScene, ascensceurs::PluginAscenseur));
    }
}
