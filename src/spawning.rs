use crate::{ai::Target, building::Door, person};
use bevy::{math::Vec3Swizzles, prelude::*, utils::Duration};
use rand::seq::IteratorRandom;

#[derive(Component, Deref, DerefMut)]
pub struct PersonSpawnTimer(Timer);

pub fn setup(mut commands: Commands) {
    commands.insert_resource(PersonSpawnTimer(Timer::new(
        Duration::from_secs_f32(1.0),
        true,
    )))
}

pub fn spawn_person(
    mut commands: Commands,
    mut timer: ResMut<PersonSpawnTimer>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    doors: Query<(&GlobalTransform, &Door)>,
) {
    timer.tick(time.delta());
    if timer.just_finished() {
        let mut rng = rand::thread_rng();
        let (from_transform, from_door) = doors.iter().choose(&mut rng).unwrap();
        let (to_transform, to_door) = doors.iter().choose(&mut rng).unwrap();
        let spawn_pos = from_transform.translation().xy() + (2.0 * from_door.get_open_dir());
        let target_pos = to_transform.translation().xy() + (2.0 * to_door.get_open_dir());
        info!(
            "Spawning person at {:?} with target {:?}!",
            spawn_pos, target_pos
        );
        let person_entity =
            person::add_person(&mut commands, &mut meshes, &mut materials, spawn_pos);
        commands.entity(person_entity).insert(Target(target_pos));
    }
}
