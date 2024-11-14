use bevy::{
    prelude::*, 
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}
};
use bevy_framepace::FramepacePlugin;

#[derive(Resource)]
pub struct Debug(bool);

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

pub struct PluginFPS;

impl Plugin for PluginFPS {
    fn build(&self, app: &mut App) {
        app.insert_resource(Debug(false));
        app.add_plugins((
            FrameTimeDiagnosticsPlugin, 
            FramepacePlugin
        ));
        app.add_systems(Startup, 
            setup);
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
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font_size: 60.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 60.0,
                ..default()
            }),
        ]),
        FpsText,
    ));
}

fn text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}