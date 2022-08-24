mod camera;
mod person;
mod player;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use person::add_person;
use player::Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(3.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO.into(),
            ..default()
        })
        .add_startup_system(camera::setup)
        .add_startup_system(game_setup)
        .add_system(player::controls)
        .add_system(person::movement)
        .run();
}

fn game_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let person_entity = add_person(&mut commands, &mut meshes, &mut materials, Vec2::ZERO);
    commands.entity(person_entity).insert(Player);

    add_person(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec2::new(10.0, 0.0),
    );
    add_person(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec2::new(10.0, 10.0),
    );
    add_person(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec2::new(10.0, -10.0),
    );
    add_person(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec2::new(10.0, 20.0),
    );
}
