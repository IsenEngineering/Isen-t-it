use bevy::prelude::*;
use bevy_framepace::FramepacePlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Resource)]
pub struct Debug(bool);

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

pub struct PluginPerf;

impl Plugin for PluginPerf {
    fn build(&self, app: &mut App) {
        app.insert_resource(Debug(true));
        app.insert_resource(LastUpdate(0.0));
        app.add_plugins(FramepacePlugin);
        app.add_plugins(WorldInspectorPlugin::new().run_if(is_debug));

        // On affiche les FPS uniquement si le jeu 
        // est en mode développement / débug
        app.add_systems(Startup, 
            setup.run_if(is_debug));
        app.add_systems(Update, 
            text_update_system.run_if(is_debug));
    }
}

pub fn is_debug(debug: Res<Debug>) -> bool {
    debug.0
}

fn setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_sections([
            TextSection::from_style(TextStyle {
                font_size: 24.0,
                ..default()
            }),
            TextSection::new(
                " ms",
                TextStyle {
                    font_size: 24.0,
                    ..default()
                },
            )
        ]),
        FpsText,
    ));
}

#[derive(Resource)]
struct LastUpdate(f64);

fn text_update_system(
    mut last_update: ResMut<LastUpdate>,
    time: Res<Time>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    if last_update.0 + 0.25 < time.elapsed_seconds_f64() {
        for mut text in &mut query {
            let tm = time.delta_seconds_f64() * 1000.;
            text.sections[0].value = format!("{tm:.2}");
        }
        last_update.0 = time.elapsed_seconds_f64();
    }
}