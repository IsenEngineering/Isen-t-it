use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::joueur::Velocity;
use crate::joueur::{PLAYER_SPEED, PLAYER_SPRINT_SPEED};

pub fn move_sprite_touches(touches: Res<Touches>,
    mut sprite_position: Query<&mut Velocity, With<Sprite>>,
    window: Query<&Window, With<PrimaryWindow>>) {
    let mut v = sprite_position.single_mut();
    let screen = window.single();
    let height = screen.height() as f32;
    let width = screen.width() as f32;
    
    for touch in touches.iter() {
        let p = touch.position();

        v.dy = if p.y > height * 0.8 {
            -PLAYER_SPEED
        } else if p.y < height * 0.2 {
            PLAYER_SPEED
        } else {
            0.0
        };

        v.dx = if p.x > width * 0.66 {
            PLAYER_SPEED
        } else if p.x < width * 0.33 {
            -PLAYER_SPEED
        } else {
            0.0
        };
    }
}

pub fn move_sprite(keyboard: Res<ButtonInput<KeyCode>>,
    mut sprite_position: Query<&mut Velocity, With<Sprite>>) {
    // Il n'y a par défaut qu'un unique sprite.
    let mut v = sprite_position.single_mut();

    // Norme du vecteur de velocité
    let norme: f32 = match keyboard.pressed(KeyCode::ShiftLeft) {
        // appuyer sur shift permet de courir plus vite
        true => PLAYER_SPRINT_SPEED,
        false => PLAYER_SPEED
    };

    // On donne un mouvement sur l'axe Y
    v.dy = 0.0;
    if keyboard.pressed(KeyCode::KeyW) {
        v.dy += norme;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        v.dy -= norme;
    } 
    
    // On donne un mouvement sur l'axe X
    v.dx = 0.0;
    if keyboard.pressed(KeyCode::KeyA) {
        v.dx -= norme;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        v.dx += norme;
    }
}