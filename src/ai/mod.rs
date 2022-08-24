pub mod path_debug;
pub mod search;

use crate::person::*;
use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::*;

#[derive(Component, Debug)]
pub struct Actions {
    steps: Vec<Action>,
    current_step: usize,
}

#[derive(Debug)]
pub enum Action {
    GoTo(Vec2),
    Despawn,
}

impl Actions {
    fn current(&self) -> Option<&Action> {
        self.steps.get(self.current_step)
    }

    fn remaining(&self) -> impl Iterator<Item = &Action> {
        self.steps.iter().skip(self.current_step)
    }

    fn next(&mut self) {
        self.current_step += 1;
    }
}

impl From<Vec<Action>> for Actions {
    fn from(steps: Vec<Action>) -> Self {
        Self {
            steps,
            current_step: 0,
        }
    }
}

#[derive(Component, Deref)]
pub struct Target(pub Vec2);

pub fn person_actions(
    mut commands: Commands,
    mut people: Query<(Entity, &mut Person, &Transform, &Actions)>,
) {
    for (person_entity, mut person, person_transform, actions) in people.iter_mut() {
        if let Some(action) = actions.current() {
            match action {
                Action::GoTo(target) => {
                    let dir = (*target - person_transform.translation.xy()).normalize_or_zero();
                    person.state = PersonState::Walking(dir);
                }
                Action::Despawn => {
                    commands.entity(person_entity).despawn();
                }
            }
        } else {
            person.state = PersonState::Standing;
        }
    }
}

pub fn path_update(mut paths: Query<(&Transform, &mut Actions)>) {
    for (person_transform, mut path) in paths.iter_mut() {
        let finished_step = if let Some(step) = path.current() {
            match step {
                Action::GoTo(target) => {
                    let distance = target.distance(person_transform.translation.xy());
                    distance < 0.5
                }
                _ => false,
            }
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
        let mut actions = vec![];
        let from = transform.translation.xy();
        let raw_path = search::search_path(&rapier_ctx, from, **target).unwrap();
        let simplified_path = path_simplification(&rapier_ctx, raw_path);
        actions.extend(
            simplified_path
                .into_iter()
                .map(|target| Action::GoTo(target)),
        );
        actions.push(Action::Despawn);

        info!("{:?} actions: {:?}", entity, actions);
        commands
            .entity(entity)
            .insert(Actions::from(actions))
            .remove::<Target>();
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
