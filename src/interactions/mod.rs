use bevy::prelude::*;
use bevy_light_2d::light::PointLight2d;

use crate::joueur::Velocity;

#[derive(Component)]
struct InteractionPoint {
    max_intensity: f32,
    intensity_variance: f32,
    min_intensity: f32,
    // La distance à laquelle l'interaction est disponible
    // et la surbrillance arrive
    distance: f32,

    // Couleur de l'interaction lorsque le joueur est proche.
}

pub struct Interactions;

impl Plugin for Interactions {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, animate_interaction);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        InteractionPoint {
            distance: 12.0,
            max_intensity: 5.0,
            min_intensity: 2.0,
            intensity_variance: 10.0
        },
        SpatialBundle {
            transform: Transform::from_translation(Vec3::new(
                260.5,
                34.0,
                1.5
            )),
            ..default()
        },
        PointLight2d {
            color: Color::linear_rgb(0.5, 0.5,1.0),
            radius: 6.0,
            intensity: 3.0,
            ..default()
        }
    )).with_children(|parent| {
        parent.spawn(Text2dBundle {
            text: Text::from_section("Press F to Interact", TextStyle {
                font_size: 12.0,
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(
                    0.0,
                    6.0,
                    0.0
                ),
                scale: Vec3::new(0.1, 0.1, 1.0),
                ..default()
            },
            ..default()
        });
    });
}

fn animate_interaction(
    time: Res<Time>,
    mut interactions_query: Query<(&mut PointLight2d, &InteractionPoint, 
        &Transform)>,
    player_query: Query<&Transform, (Changed<Transform>, With<Velocity>)>) {
    for player in player_query.iter() {
        for (mut light, point, 
            transform) 
            in interactions_query.iter_mut() {
            if player.translation.distance(transform.translation) > point.distance {
                if light.intensity > point.min_intensity {
                    light.intensity -= point.intensity_variance * time.delta_seconds();
                }
                continue;
            }
            if light.intensity < point.max_intensity {
                light.intensity += point.intensity_variance * time.delta_seconds();
            }
        }
    }
}