use crate::person::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn controls(keyboard: Res<Input<KeyCode>>, mut player: Query<&mut Person, With<Player>>) {
    let dir = get_direction(&keyboard);
    let mut player = player.single_mut();
    if dir.length() > 0.1 {
        player.state = PersonState::Walking(dir);
    } else {
        player.state = PersonState::Standing;
    }
}

fn get_direction(keyboard: &Input<KeyCode>) -> Vec2 {
    let mut dir = Vec2::ZERO;
    if keyboard.pressed(KeyCode::Up) {
        dir.y += 1.0;
    }
    if keyboard.pressed(KeyCode::Down) {
        dir.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::Right) {
        dir.x += 1.0;
    }
    if keyboard.pressed(KeyCode::Left) {
        dir.x -= 1.0;
    }
    dir.normalize_or_zero()
}
