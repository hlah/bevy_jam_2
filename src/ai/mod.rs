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

    fn peek(&self) -> Option<&Action> {
        self.steps.get(self.current_step + 1)
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

#[derive(Component, Deref, Debug)]
pub struct Target(pub Vec2);

#[derive(Component, Debug)]
pub struct BuildPath;

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

pub fn path_update(
    mut commands: Commands,
    rapier_ctx: Res<RapierContext>,
    mut transform_and_actions: Query<(Entity, &Transform, &mut Actions)>,
) {
    for (entity, transform, mut actions) in transform_and_actions.iter_mut() {
        rebuild_actions_if_stuck(&mut commands, &rapier_ctx, entity, transform, &mut actions);
        check_step_finshed(&rapier_ctx, transform, &mut actions);
    }
}

fn check_step_finshed(rapier_ctx: &RapierContext, transform: &Transform, actions: &mut Actions) {
    let finished_step = if let Some(Action::GoTo(target)) = actions.current() {
        let pos = transform.translation.xy();

        if let Some(Action::GoTo(next_target)) = actions.peek() {
            let displacement = *next_target - pos;

            let distance = displacement.length();
            let dir = displacement.normalize_or_zero();
            let result = rapier_ctx.cast_shape(
                pos,
                0.0,
                dir,
                &Collider::cuboid(0.5, 0.5),
                distance,
                QueryFilter::only_fixed(),
            );
            result.is_none()
        } else {
            pos.distance(*target) < 0.5
        }
    } else {
        false
    };
    if finished_step {
        actions.next();
    }
}

fn rebuild_actions_if_stuck(
    commands: &mut Commands,
    rapier_ctx: &RapierContext,
    entity: Entity,
    transform: &Transform,
    actions: &mut Actions,
) {
    let rebuild = if let Some(Action::GoTo(target)) = actions.current() {
        let pos = transform.translation.xy();
        let displacement = *target - pos;
        let distance = displacement.length();
        let dir = displacement.normalize_or_zero();
        let result = rapier_ctx.cast_shape(
            pos,
            0.0,
            dir,
            &Collider::cuboid(0.5, 0.5),
            distance,
            QueryFilter::only_fixed(),
        );
        result.is_some()
    } else {
        false
    };
    if rebuild {
        info!("Rebuilding path for {:?}", entity);
        commands.entity(entity).insert(BuildPath);
    }
}

pub fn build_path(
    mut commands: Commands,
    rapier_ctx: Res<RapierContext>,
    to_build: Query<(Entity, &Transform, &Target), With<BuildPath>>,
) {
    for (entity, transform, target) in to_build.iter() {
        let mut actions = vec![];
        let from = transform.translation.xy();
        let raw_path = search::search_path(&rapier_ctx, from, **target).unwrap();
        let simplified_path = path_simplification(&rapier_ctx, raw_path);
        actions.extend(simplified_path.into_iter().map(Action::GoTo));
        actions.push(Action::Despawn);

        commands
            .entity(entity)
            .insert(Actions::from(actions))
            .remove::<BuildPath>();
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
        .cast_shape(
            from,
            0.0,
            dir,
            &Collider::cuboid(0.5, 0.5),
            from.distance(to),
            QueryFilter::only_fixed(),
        )
        .is_none()
}
