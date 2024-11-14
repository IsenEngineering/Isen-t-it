use bevy::prelude::*;
use crate::composants::Velocity;

const SCENE_FILE_PATH: &str = "maquette.png";


pub struct PluginScene;

impl Plugin for PluginScene {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, stairs);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    // Les animations du personnage
    let texture = asset_server.load(SCENE_FILE_PATH);

    // On spawn l'entité
    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform {
                // On la dépose vers le centre
                translation: Vec3::new(240.0, 168.0, 0.),
                // On double sa taille
                scale: Vec3::new(1., 1., 1.),
                ..default()
            },
            ..default()
        },
    ));
}

fn stairs(keyboard: Res<ButtonInput<KeyCode>>,
    mut transforms: ParamSet<(
        Query<&mut Transform, (With<Sprite>, With<Velocity>)>,
        Query<&mut Transform, With<Camera>>
    )>) {
    let mut v = transforms.p0().single_mut().translation;
    let mut cam = transforms.p1().single_mut().translation;
    
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        v.y += 48.0;
        cam.y += 48.0;
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        v.y -= 48.0;
        cam.y -= 48.0;
    }
}