use bevy::prelude::*;
use bevy_prototype_lyon::prelude::{FillMode, *};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Building {
    pub size: Vec2,
    pub pos: Vec2,
    pub doors: Vec<Door>,
}

#[derive(Debug, Clone, Copy)]
pub enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

impl Side {
    fn get_pos(&self, building_size: Vec2, pos: f32) -> Vec2 {
        0.5 * match self {
            Self::Left => Vec2::new(-building_size.x, pos * building_size.y),
            Self::Right => Vec2::new(building_size.x, pos * building_size.y),
            Self::Bottom => Vec2::new(pos * building_size.x, -building_size.y),
            Self::Top => Vec2::new(pos * building_size.x, building_size.y),
        }
    }

    fn get_offet_dir(&self) -> Vec2 {
        match self {
            Self::Left => Vec2::Y,
            Self::Right => -Vec2::Y,
            Self::Bottom => -Vec2::X,
            Self::Top => Vec2::X,
        }
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Door {
    pub side: Side,
    pos: f32,
}

impl Door {
    pub fn new(side: Side, pos: f32) -> Self {
        Self { side, pos }
    }

    pub fn get_open_dir(&self) -> Vec2 {
        match self.side {
            Side::Left => -Vec2::X,
            Side::Right => Vec2::X,
            Side::Bottom => -Vec2::Y,
            Side::Top => Vec2::Y,
        }
    }
}

pub fn on_add_building(
    mut commands: Commands,
    added_building: Query<(Entity, &Building), Added<Building>>,
) {
    for (building_entity, building) in added_building.iter() {
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

        draw_doors(&mut commands, building_entity, building);
    }
}

fn draw_doors(commands: &mut Commands, building_entity: Entity, building: &Building) {
    for door in &building.doors {
        let pos = door.side.get_pos(building.size, door.pos);
        let offset_dir = door.side.get_offet_dir();
        info!("Putting door at {:?}", pos);
        commands.entity(building_entity).add_children(|children| {
            children
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Line(offset_dir * 2.0, -offset_dir * 2.0),
                    DrawMode::Stroke(StrokeMode::new(Color::MAROON, 0.5)),
                    Transform::from_xyz(pos.x, pos.y, 5.0),
                ))
                .insert(*door);
        });
    }
}
