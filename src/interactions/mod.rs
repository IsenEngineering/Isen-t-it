use bevy::prelude::*;
use bevy_light_2d::light::PointLight2d;

use crate::joueur::composants::Velocity;

#[derive(Component)]
pub struct InteractionPoint {
    // Intensité lorsque l'utilisateur peut intéragir
    max_intensity: f32,

    // Vitesse du changement d'intensité
    intensity_variance: f32,

    // Intensité lorsque l'utilisateur ne peut intéragir
    min_intensity: f32,

    // La distance à laquelle l'interaction est disponible
    // et la surbrillance survient.
    distance: f32,
}

impl Default for InteractionPoint {
    fn default() -> Self {
        InteractionPoint {
            max_intensity: 3.0,
            intensity_variance: 10.0,
            min_intensity: 0.5,
            distance: 24.0
        }
    }
}

#[derive(Bundle, Default)]
pub struct InteractionBundle {
    // Les informations sur le point d'interaction
    pub interaction_point: InteractionPoint,

    // Le point de lumière
    pub light_point: PointLight2d,

    /// The visibility of the entity.
    pub visibility: Visibility,

    /// The inherited visibility of the entity.
    pub inherited_visibility: InheritedVisibility,

    /// The view visibility of the entity.
    pub view_visibility: ViewVisibility,

    /// The transform of the entity.
    pub transform: Transform,

    /// The global transform of the entity.
    pub global_transform: GlobalTransform,
}

pub struct Interactions;

impl Plugin for Interactions {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, animate_interaction);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(InteractionBundle {
        interaction_point: InteractionPoint {
            distance: 12.0,
            max_intensity: 2.0,
            min_intensity: 0.5,
            intensity_variance: 5.0,
        },
        light_point: PointLight2d {
            color: Color::linear_rgb(0.5, 0.5, 2.0),
            radius: 12.0,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(260.5, 34.0, 3.0)),
        ..default()
    })
        .with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    "Press F to Interact",
                    TextStyle {
                        font_size: 12.0,
                        ..default()
                    },
                ),
                transform: Transform {
                    translation: Vec3::new(0.0, 6.0, 0.0),
                    scale: Vec3::new(0.1, 0.1, 1.0),
                    ..default()
                },
                ..default()
            });
        });
}

fn animate_interaction(
    time: Res<Time>,
    mut interactions_query: Query<(&mut PointLight2d, &InteractionPoint, &Transform)>,
    player_query: Query<&Transform, (Changed<Transform>, With<Velocity>)>,
) {
    // On passe sur chaque joueur
    for player in player_query.iter() {
        // On passe sur chaque point de lumière
        for (mut light, point, transform) in interactions_query.iter_mut() {
            // Si le point est trop loin du joueur on ne fait rien
            // (à l'exception de la correction d'intensité)
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
