use bevy::{
    diagnostic::DiagnosticsStore, 
    prelude::*
};
use super::FrameTimeDiagnosticsPlugin;

#[derive(Component)]
pub struct DebugFrame;

pub fn update(mut text: Query<&mut Text, With<DebugFrame>>,
    diagnostics: Res<DiagnosticsStore>) {
    let fps = match diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        Some(v) => v,
        None => return,
    };

    for mut t in text.iter_mut() {
        t.sections[0].value = format!(
            "{:.1} FPS", 
            fps.smoothed().unwrap_or(0.0)
        );
    }
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Inter.ttf");

    // root node
    commands
    .spawn(NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::End,

            position_type: PositionType::Absolute,
            margin: UiRect::all(Val::Px(25.)),
            bottom: Val::Px(0.),
            right: Val::Px(0.),
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        let style = TextStyle {
            font: font.clone(),
            color: Color::linear_rgb(0.1, 0.1, 0.1),
            font_size: 12.0,
            ..default()
        };
        parent.spawn((
            TextBundle::from_section(
                format!("Isen't It - {}", VERSION), 
                style.clone()
            ),
            Label
        ));
        parent.spawn((
            TextBundle::from_section(
                "", 
                style
            ),
            Label,
            DebugFrame
        ));
    });
}