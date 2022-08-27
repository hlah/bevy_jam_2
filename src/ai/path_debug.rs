use super::{Action, Actions};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use itertools::Itertools;

#[derive(Component, Deref)]
pub struct PathDebugRef(Entity);

#[derive(Component, Deref)]
pub struct PathDebug(Entity);

#[allow(dead_code)]
pub fn path_debug(
    mut commands: Commands,
    changed_paths: Query<(Entity, &Actions, Option<&PathDebugRef>), Changed<Actions>>,
) {
    for (entity, path, path_debug) in changed_paths.iter() {
        if let Some(path_debug) = path_debug {
            commands.entity(**path_debug).despawn();
        }

        let mut gb = GeometryBuilder::new();
        for (from, to) in path
            .remaining()
            .filter_map(|action| {
                if let Action::GoTo(target) = action {
                    Some(target)
                } else {
                    None
                }
            })
            .tuple_windows()
        {
            gb = gb.add(&shapes::Line(*from, *to));
        }
        let path_debug_entity = commands
            .spawn_bundle(gb.build(
                DrawMode::Stroke(StrokeMode::new(Color::RED, 0.1)),
                Transform::default(),
            ))
            .insert(PathDebug(entity))
            .id();

        commands
            .entity(entity)
            .insert(PathDebugRef(path_debug_entity));
    }
}
