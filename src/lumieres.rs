use bevy::{core_pipeline::bloom::Bloom, prelude::*};
use bevy_light_2d::{light::{AmbientLight2d, PointLight2d}, plugin::Light2dPlugin};

pub struct PluginLumieres;

impl Plugin for PluginLumieres {
    // #[cfg(not(target_arch = "wasm32"))]
    fn build(&self, app: &mut App) {
        app.add_plugins(Light2dPlugin);

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
            AmbientLight2d {
                color: Color::WHITE,
                brightness: 0.05
            }
        ));
    }

    for i in 0..10 {
        let mut lumiere = PointLight2d::default();
        lumiere.radius = 72.0;
        lumiere.intensity = 1.0;

        commands.spawn((
            lumiere,
            Name::new(format!("Lumière n{}", i + 1)),
            Transform::from_translation(
                Vec3::new(24.0 + i as f32 * 48.0, 72.0, 24.0)
            )
        ));
    }
}
