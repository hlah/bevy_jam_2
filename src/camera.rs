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
