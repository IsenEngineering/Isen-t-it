use bevy::{
    prelude::*,
    core_pipeline::bloom::BloomSettings
};
use bevy_light_2d::prelude::*;

pub struct PluginLumieres;

impl Plugin for PluginLumieres {
    fn build(&self, app: &mut App) {
        app.add_plugins(Light2dPlugin);
        app.add_systems(PostStartup, setup);
    }
}

fn setup(mut commands: Commands, cameras: Query<Entity, With<Camera>>) {
    for i in 0..5 {
        commands.spawn(PointLight2dBundle {
            point_light: PointLight2d {
                radius: 72.0,
                intensity: 2.,
                cast_shadows: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(24. + 96. * i as f32, 72., 5.),
                ..default()
            },
            ..default()
        });
    }

    for entity in cameras.iter() {
        commands.entity(entity).insert((
            AmbientLight2d {
                brightness: 0.1,
                ..default()
            },
            BloomSettings::OLD_SCHOOL
        ));
    }
}