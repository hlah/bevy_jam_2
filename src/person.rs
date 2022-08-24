use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
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

pub fn movement(mut persons: Query<(&mut ExternalImpulse, &Velocity, &Person)>) {
    for (mut impulse, velocity, person) in persons.iter_mut() {
        match person.state {
            PersonState::Walking(dir) => {
                let current_dir = velocity.linvel.normalize_or_zero();
                let impulse_dir = (2.0 * dir - current_dir).normalize_or_zero();
                impulse.impulse = 10.0 * impulse_dir;
            }
            PersonState::Standing => {
                impulse.impulse = -20.0 * velocity.linvel;
            }
        }
    }
}
