use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Component)]
pub struct Road {
    pub from: Entity,
    pub to: Entity,
}

#[derive(Component)]
pub struct RoadNode {
    pub pos: Vec2,
}

pub fn on_add_road(
    mut commands: Commands,
    added_road: Query<(Entity, &Road), Added<Road>>,
    road_nodes: Query<&RoadNode>,
) {
    for (road_entity, road) in added_road.iter() {
        let from_pos = road_nodes.get(road.from).unwrap().pos;
        let to_pos = road_nodes.get(road.to).unwrap().pos;

        let line = shapes::Line(from_pos, to_pos);
        commands
            .entity(road_entity)
            .insert_bundle(GeometryBuilder::build_as(
                &line,
                DrawMode::Stroke(StrokeMode::new(Color::DARK_GRAY, 20.0)),
                Transform::from_xyz(0.0, 0.0, -10.0),
            ));
    }
}

pub fn on_add_road_node(
    mut commands: Commands,
    added_road_node: Query<(Entity, &RoadNode), Added<RoadNode>>,
) {
    for (node_entity, node) in added_road_node.iter() {
        let square = shapes::Rectangle {
            extents: Vec2::ONE * 20.0,
            origin: RectangleOrigin::Center,
        };
        commands
            .entity(node_entity)
            .insert_bundle(GeometryBuilder::build_as(
                &square,
                DrawMode::Fill(FillMode::color(Color::DARK_GRAY)),
                Transform::from_xyz(node.pos.x, node.pos.y, -10.0),
            ));
    }
}
