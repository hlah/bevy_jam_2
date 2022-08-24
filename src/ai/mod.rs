pub mod path_debug;
pub mod search;

use crate::person::*;
use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::*;

#[derive(Component, Debug)]
pub struct Path {
    steps: Vec<Vec2>,
    current_step: usize,
}

impl Path {
    fn current(&self) -> Option<&Vec2> {
        self.steps.get(self.current_step)
    }

    fn remaining(&self) -> &[Vec2] {
        &self.steps[self.current_step..]
    }

    fn next(&mut self) {
        self.current_step += 1;
    }
}

impl From<Vec<Vec2>> for Path {
    fn from(steps: Vec<Vec2>) -> Self {
        Self {
            steps,
            current_step: 0,
        }
    }
}

#[derive(Component, Deref)]
pub struct Target(pub Vec2);

pub fn person_movement(mut people: Query<(&mut Person, &Transform, &Path)>) {
    for (mut person, person_transform, path) in people.iter_mut() {
        if let Some(target) = path.current() {
            let dir = (*target - person_transform.translation.xy()).normalize_or_zero();
            person.state = PersonState::Walking(dir);
        } else {
            person.state = PersonState::Standing;
        }
    }
}

pub fn path_update(mut paths: Query<(&Transform, &mut Path)>) {
    for (person_transform, mut path) in paths.iter_mut() {
        let finished_step = if let Some(target) = path.current() {
            let distance = target.distance(person_transform.translation.xy());
            distance < 0.5
        } else {
            false
        };
        if finished_step {
            path.next();
        }
    }
}

pub fn build_path(
    mut commands: Commands,
    rapier_ctx: Res<RapierContext>,
    with_target: Query<(Entity, &Transform, &Target)>,
) {
    for (entity, transform, target) in with_target.iter() {
        let from = transform.translation.xy();
        let raw_path = search::search_path(&rapier_ctx, from, **target).unwrap();
        let path = Path::from(path_simplification(&rapier_ctx, raw_path));
        info!("{:?} path: {:?}", entity, path);
        commands.entity(entity).insert(path).remove::<Target>();
    }
}

fn path_simplification(rapier_ctx: &RapierContext, path: Vec<Vec2>) -> Vec<Vec2> {
    let mut simplified_path = vec![path[0]];
    let mut i = 1;
    while i < path.len() - 1 {
        if !can_see(rapier_ctx, *simplified_path.last().unwrap(), path[i + 1]) {
            simplified_path.push(path[i])
        } else {
        }
        i += 1;
    }

    simplified_path.push(*path.last().unwrap());
    simplified_path
}

fn can_see(rapier_ctx: &RapierContext, from: Vec2, to: Vec2) -> bool {
    let dir = (to - from).normalize();
    rapier_ctx
        .cast_ray(
            from,
            dir,
            from.distance(to),
            true,
            QueryFilter::only_fixed(),
        )
        .is_none()
}
