use bevy::{
    prelude::*,
    core_pipeline::bloom::BloomSettings
};
use bevy_light_2d::prelude::*;

pub struct PluginLumieres;

impl Plugin for PluginLumieres {
    #[cfg(not(target_arch = "wasm32"))]
    fn build(&self, app: &mut App) {
        app.add_plugins(Light2dPlugin);
        app.add_systems(PostStartup, setup);
    }
    #[cfg(target_arch = "wasm32")]
    fn build(&self, _: &mut App) {
        // On ne mets pas les lumières sur les clients web 
        // (le jeu version web est très peu performant)
    }
}

fn setup(mut commands: Commands, cameras: Query<Entity, With<Camera>>) {
    for i in 0..5 {
        // On mets 5 lumières sur le première étage
        commands.spawn(PointLight2dBundle {
            point_light: PointLight2d {
                // La distance que couvre la lumière
                radius: 72.0,
                // L'intensité de la lumière
                intensity: 2.,
                cast_shadows: true,
                ..default()
            },
            transform: Transform {
                // 24. <- au milieu; 96. * i <- une lumière tous les deux carrées
                // 5. en z pour qu'elles soient au dessus du fond.
                translation: Vec3::new(24. + 96. * i as f32, 72., 5.),
                ..default()
            },
            ..default()
        });
    }

    for entity in cameras.iter() {
        // On ajoute à la caméra un effet de bloom 
        // et une lumière ambiante faible pour pas être dans le noir.
        commands.entity(entity).insert((
            AmbientLight2d {
                brightness: 0.1,
                ..default()
            },
            BloomSettings::OLD_SCHOOL
        ));
    }
}