use bevy::{core_pipeline::bloom::Bloom, prelude::*};

pub struct PluginLumieres;

impl Plugin for PluginLumieres {
    // #[cfg(not(target_arch = "wasm32"))]
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup);
    }
    // #[cfg(target_arch = "wasm32")]
    // fn build(&self, _: &mut App) {
    //     // On ne mets pas les lumières sur les clients web
    //     // (le jeu version web est très peu performant)
    // }
}

fn setup(mut commands: Commands, cameras: Query<Entity, With<Camera>>) {
    for entity in cameras.iter() {
        // On ajoute à la caméra un effet de bloom
        // et une lumière ambiante faible pour pas être dans le noir.
        commands.entity(entity).insert((
            Bloom::OLD_SCHOOL,
        ));
    }
}
