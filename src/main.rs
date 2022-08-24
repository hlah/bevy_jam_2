mod ai;
mod building;
mod camera;
mod controls;
mod person;
mod player;
mod road;
mod spawning;

use ai::Target;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::*;
use building::*;
use person::add_person;
use player::Player;
use road::*;

#[derive(SystemLabel)]
enum SystemLabels {
    PathUpdate,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(3.0))
        //.add_plugin(RapierDebugRenderPlugin::default())
        //.add_plugin(EditorPlugin)
        .add_plugin(ShapePlugin)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_startup_system(camera::setup)
        .add_startup_system(game_setup)
        .add_startup_system(spawning::setup)
        .add_system(controls::player_movement)
        .add_system(controls::camera_zoom)
        .add_system(person::movement)
        .add_system(camera::follow_player)
        .add_system(road::on_add_road)
        .add_system(road::on_add_road_node)
        .add_system(building::on_add_building)
        .add_system(ai::path_update.label(SystemLabels::PathUpdate))
        .add_system(ai::person_movement.after(SystemLabels::PathUpdate))
        .add_system_to_stage(CoreStage::PostUpdate, ai::build_path)
        //.add_system(ai::path_debug::path_debug)
        .add_system(spawning::spawn_person)
        .run();
}

fn game_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let person_entity = add_person(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec2::new(0.0, 0.0),
    );
    commands.entity(person_entity).insert(Player);

    add_roads(&mut commands);
    add_buildings(&mut commands);
    add_people(&mut commands, &mut meshes, &mut materials);
}

pub fn add_roads(commands: &mut Commands) {
    let node_a = commands
        .spawn()
        .insert(RoadNode {
            pos: Vec2::new(-100.0, 100.0),
        })
        .id();
    let node_b = commands
        .spawn()
        .insert(RoadNode {
            pos: Vec2::new(-100.0, -100.0),
        })
        .id();
    let node_c = commands
        .spawn()
        .insert(RoadNode {
            pos: Vec2::new(100.0, -100.0),
        })
        .id();
    let node_d = commands
        .spawn()
        .insert(RoadNode {
            pos: Vec2::new(100.0, 100.0),
        })
        .id();

    commands.spawn().insert(Road {
        from: node_a,
        to: node_b,
    });
    commands.spawn().insert(Road {
        from: node_b,
        to: node_c,
    });
    commands.spawn().insert(Road {
        from: node_c,
        to: node_d,
    });
    commands.spawn().insert(Road {
        from: node_d,
        to: node_a,
    });
}

pub fn add_buildings(commands: &mut Commands) {
    commands.spawn().insert(Building {
        pos: Vec2::new(-50.0, -50.0),
        size: Vec2::new(50.0, 50.0),
        doors: vec![Door::new(Side::Left, 0.0)],
    });
    commands.spawn().insert(Building {
        pos: Vec2::new(50.0, -50.0),
        size: Vec2::new(50.0, 50.0),
        doors: vec![Door::new(Side::Bottom, 0.0)],
    });
    commands.spawn().insert(Building {
        pos: Vec2::new(50.0, 50.0),
        size: Vec2::new(50.0, 50.0),
        doors: vec![Door::new(Side::Top, 0.5)],
    });
    commands.spawn().insert(Building {
        pos: Vec2::new(-50.0, 50.0),
        size: Vec2::new(50.0, 50.0),
        doors: vec![Door::new(Side::Left, 0.5)],
    });

    commands.spawn().insert(Building {
        pos: Vec2::new(-50.0, 0.0),
        size: Vec2::new(50.0, 30.0),
        doors: vec![Door::new(Side::Right, 0.0)],
    });
    commands.spawn().insert(Building {
        pos: Vec2::new(50.0, 0.0),
        size: Vec2::new(50.0, 30.0),
        doors: vec![Door::new(Side::Right, 0.0)],
    });

    commands.spawn().insert(Building {
        pos: Vec2::new(0.0, 50.0),
        size: Vec2::new(30.0, 50.0),
        doors: vec![Door::new(Side::Bottom, 0.5)],
    });
    commands.spawn().insert(Building {
        pos: Vec2::new(0.0, -50.0),
        size: Vec2::new(30.0, 50.0),
        doors: vec![],
    });
}

pub fn add_people(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    let person_entity = add_person(commands, meshes, materials, Vec2::new(62.5, 77.0));
    commands
        .entity(person_entity)
        .insert(Target(Vec2::new(-77.0, 62.5)));
}
