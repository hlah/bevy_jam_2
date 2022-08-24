use crate::player::Player;
use bevy::{prelude::*, render::camera::ScalingMode};

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(100.0),
            near: -1000.0,
            far: 1000.0,
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
    let player_translation = player.single().translation;
    camera_transform.translation = Vec3::new(player_translation.x, player_translation.y, 0.0);
}
