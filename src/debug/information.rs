use super::FrameTimeDiagnosticsPlugin;
use aeronet::io::Session;
use bevy::{diagnostic::DiagnosticsStore, prelude::*};

#[derive(Component)]
pub struct DebugFrame;

#[derive(Component)]
pub struct ConnectionStateFrame;

pub fn update(mut text: Query<&mut Text, With<DebugFrame>>, diagnostics: Res<DiagnosticsStore>) {
    let fps = match diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        Some(v) => v,
        None => return,
    };

    for mut t in text.iter_mut() {
        t.0 = format!("{:.1} FPS", fps.smoothed().unwrap_or(0.0));
    }
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Inter.ttf");

    // root node
    commands
        .spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::End,

            position_type: PositionType::Absolute,
            margin: UiRect::all(Val::Px(25.)),
            bottom: Val::Px(0.),
            right: Val::Px(0.),

            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text("Hors ligne".to_string()),
                TextColor(Color::linear_rgb(0.2, 0.1, 0.1)),
                TextFont {
                    font: font.clone(),
                    font_size: 12.0,
                    ..default()
                },
                ConnectionStateFrame
            ));
            let text = format!("Isen't It - {}", VERSION);
            parent.spawn((
                Text(text),
                TextColor(Color::linear_rgb(0.1, 0.1, 0.1)),
                TextFont {
                    font: font.clone(),
                    font_size: 12.0,
                    ..default()
                },
            ));
            parent.spawn((
                Text("".to_string()),
                TextColor(Color::linear_rgb(0.1, 0.1, 0.1)),
                TextFont {
                    font: font.clone(),
                    font_size: 12.0,
                    ..default()
                },
                DebugFrame,
            ));
        });
}

pub fn connected(
    _: Trigger<OnAdd, Session>,
    mut texts: Query<(&mut Text, &mut TextColor), With<ConnectionStateFrame>>
) {
    for (mut text, mut color) in texts.iter_mut() {
        text.0 = "En ligne".to_string();
        color.0 = Color::linear_rgb(0.1, 0.2, 0.1);
    }
}

pub fn disconnected(
    _: Trigger<OnRemove, Session>,
    mut texts: Query<(&mut Text, &mut TextColor), With<ConnectionStateFrame>>
) {
    for (mut text, mut color) in texts.iter_mut() {
        text.0 = "Hors ligne".to_string();
        color.0 = Color::linear_rgb(0.2, 0.1, 0.1);
    }
}