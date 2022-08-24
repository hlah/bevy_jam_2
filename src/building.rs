use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{FillMode, *};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Building {
    pub size: Vec2,
    pub pos: Vec2,
}

pub fn on_add_building(
    mut commands: Commands,
    added_road_node: Query<(Entity, &Building), Added<Building>>,
) {
    for (building_entity, building) in added_road_node.iter() {
        let square = shapes::Rectangle {
            extents: building.size,
            origin: RectangleOrigin::Center,
        };
        commands
            .entity(building_entity)
            .insert(RigidBody::Fixed)
            .insert(Collider::cuboid(
                building.size.x / 2.0,
                building.size.y / 2.0,
            ))
            .insert_bundle(GeometryBuilder::build_as(
                &square,
                DrawMode::Fill(FillMode::color(Color::GRAY)),
                Transform::from_xyz(building.pos.x, building.pos.y, 0.0),
            ));
    }
}
