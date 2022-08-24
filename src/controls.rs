use crate::{person::*, player::Player};
use bevy::prelude::*;

const UP_KEY: KeyCode = KeyCode::Comma;
const DOWN_KEY: KeyCode = KeyCode::O;
const LEFT_KEY: KeyCode = KeyCode::A;
const RIGHT_KEY: KeyCode = KeyCode::E;
const ZOOM_OUT: KeyCode = KeyCode::K;
const ZOOM_IN: KeyCode = KeyCode::J;

pub fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut player: Query<&mut Person, With<Player>>,
) {
    let dir = get_direction(&keyboard);
    let mut player = player.single_mut();
    if dir.length() > 0.1 {
        player.state = PersonState::Walking(dir);
    } else {
        player.state = PersonState::Standing;
    }
}

pub fn camera_zoom(
    keyboard: Res<Input<KeyCode>>,
    mut camera: Query<&mut OrthographicProjection, With<Camera>>,
) {
    let mut projection = camera.single_mut();
    if keyboard.just_pressed(ZOOM_OUT) {
        projection.scale = 2.0_f32.min(projection.scale + 0.2);
    }
    if keyboard.just_pressed(ZOOM_IN) {
        projection.scale = 0.2_f32.max(projection.scale - 0.2);
    }
}

fn get_direction(keyboard: &Input<KeyCode>) -> Vec2 {
    let mut dir = Vec2::ZERO;
    if keyboard.pressed(UP_KEY) {
        dir.y += 1.0;
    }
    if keyboard.pressed(DOWN_KEY) {
        dir.y -= 1.0;
    }
    if keyboard.pressed(RIGHT_KEY) {
        dir.x += 1.0;
    }
    if keyboard.pressed(LEFT_KEY) {
        dir.x -= 1.0;
    }
    dir.normalize_or_zero()
}
