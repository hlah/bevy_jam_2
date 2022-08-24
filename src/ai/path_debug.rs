use super::Path;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Component, Deref)]
pub struct PathDebug(Entity);

#[allow(dead_code)]
pub fn path_debug(
    mut commands: Commands,
    changed_paths: Query<(Entity, &Path, Option<&PathDebug>), Changed<Path>>,
) {
    for (entity, path, path_debug) in changed_paths.iter() {
        if let Some(path_debug) = path_debug {
            commands.entity(**path_debug).despawn();
        }

        let mut gb = GeometryBuilder::new();
        for window in path.remaining().windows(2) {
            gb = gb.add(&shapes::Line(window[0], window[1]));
        }
        let path_debug_entity = commands
            .spawn_bundle(gb.build(
                DrawMode::Stroke(StrokeMode::new(Color::RED, 0.1)),
                Transform::default(),
            ))
            .id();

        commands.entity(entity).insert(PathDebug(path_debug_entity));
    }
}
