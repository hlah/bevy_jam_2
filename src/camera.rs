use crate::player::Player;
use bevy::{prelude::*, render::camera::ScalingMode};

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(100.0),
            ..default()
        },
        ..default()
    });
}

pub fn follow_player(
    mut camera: Query<&mut Transform, With<Camera>>,
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera_transform = camera.single_mut();
    let player_transform = player.single();
    camera_transform.translation = player_transform.translation;
}
