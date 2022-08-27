use std::f32::consts::PI;

use bevy::{math::Vec3Swizzles, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

#[derive(Component, Default)]
pub struct Person {
    pub state: PersonState,
}

#[derive(Default, Debug)]
pub enum PersonState {
    #[default]
    Standing,
    Walking(Vec2),
}

pub fn add_person(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    pos: Vec2,
) -> Entity {
    commands
        .spawn()
        .insert(Person::default())
        .insert(RigidBody::Dynamic)
        .insert(ExternalImpulse::default())
        .insert(Velocity::zero())
        .insert(Collider::cuboid(0.5, 0.5))
        .insert(ColliderMassProperties::Mass(60.0))
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Max,
        })
        .insert(Damping {
            linear_damping: 0.99,
            ..default()
        })
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            material: materials.add(ColorMaterial::from(Color::FUCHSIA)),
            transform: Transform::from_xyz(pos.x, pos.y, 20.0),
            ..default()
        })
        .id()
}

pub fn movement(
    rapier_ctx: Res<RapierContext>,
    mut persons: Query<(Entity, &mut ExternalImpulse, &Velocity, &Person)>,
    transforms: Query<&Transform>,
    velocities: Query<&Velocity>,
) {
    for (entity, mut impulse, velocity, person) in persons.iter_mut() {
        match person.state {
            PersonState::Walking(target_dir) => {
                let current_dir = velocity.linvel.normalize_or_zero();
                let collision_avoidance_dir = calculate_collision_avoidance_dir(
                    entity,
                    target_dir,
                    &rapier_ctx,
                    &transforms,
                    &velocities,
                );
                let personal_distance_dir =
                    calculate_personal_space_dir(entity, target_dir, &rapier_ctx, &transforms);
                let total_dir = target_dir + collision_avoidance_dir + personal_distance_dir;
                let correction_dir = total_dir - current_dir;
                let impulse_dir = (total_dir + 2.0 * correction_dir).normalize_or_zero();
                impulse.impulse = 10.0 * impulse_dir;
            }
            PersonState::Standing => {
                impulse.impulse = -20.0 * velocity.linvel;
            }
        }
    }
}

fn calculate_collision_avoidance_dir(
    entity: Entity,
    target_dir: Vec2,
    rapier_ctx: &RapierContext,
    transforms: &Query<&Transform>,
    velocities: &Query<&Velocity>,
) -> Vec2 {
    let current_pos = transforms.get(entity).unwrap().translation.xy();
    let max_toi = 20.0;
    if let Some((collider_entity, toi)) = rapier_ctx.cast_shape(
        current_pos,
        0.0,
        target_dir,
        &Collider::cuboid(0.6, 0.6),
        max_toi,
        QueryFilter::exclude_fixed()
            .exclude_sensors()
            .exclude_collider(entity),
    ) {
        let collider_vel = velocities.get(collider_entity).unwrap().linvel;
        let entity_vel = velocities.get(entity).unwrap().linvel;
        let relative_vel_dir = (collider_vel - entity_vel).normalize_or_zero();
        let mut collider_side_dir = relative_vel_dir.reject_from(target_dir);
        if collider_side_dir.length() < 0.1 {
            let collider_pos = transforms.get(collider_entity).unwrap().translation.xy();
            let collider_relative_pos = current_pos - collider_pos;
            collider_side_dir = -collider_relative_pos.reject_from(target_dir);
        }
        if collider_side_dir.length() < 0.1 {
            collider_side_dir = target_dir.perp();
        }

        -(2.0 * (max_toi - toi.toi) / max_toi + 0.1) * collider_side_dir.normalize()
    } else {
        Vec2::ZERO
    }
}

fn calculate_personal_space_dir(
    entity: Entity,
    target_dir: Vec2,
    rapier_ctx: &RapierContext,
    transforms: &Query<&Transform>,
) -> Vec2 {
    let mut personal_distance_dir = Vec2::ZERO;
    let rays = 6;
    let target_dir = target_dir.normalize();

    let current_pos = transforms.get(entity).unwrap().translation.xy();
    for ray_index in 0..rays {
        let ray_dir = target_dir.rotate(Vec2::from_angle(
            2.0 * PI * (ray_index as f32 / rays as f32),
        ));
        if let Some((_, toi)) = rapier_ctx.cast_ray(
            current_pos,
            ray_dir,
            5.0,
            true,
            QueryFilter::default()
                .exclude_sensors()
                .exclude_collider(entity),
        ) {
            let dist = toi - 0.4;
            personal_distance_dir -= ray_dir / (dist * dist);
        }
    }

    personal_distance_dir
}
