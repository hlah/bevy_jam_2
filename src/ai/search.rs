use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;

pub fn search_path(rapier_ctx: &RapierContext, from: Vec2, to: Vec2) -> Option<Vec<Vec2>> {
    let mut closed = HashMap::new();
    let mut open = PriorityQueue::new();

    open.push(
        (Vec2Wrapper(from), Vec2Wrapper(from)),
        Reverse(OrderedFloat(from.distance(to))),
    );
    while let Some(((node, parent), _)) = open.pop() {
        if closed.contains_key(&node) {
            continue;
        }
        closed.insert(node, parent);

        if node.distance(to) <= 0.8 {
            return Some(build_path(node, from, closed));
        }

        for neighboor in neighboors(*node, rapier_ctx) {
            open.push(
                (Vec2Wrapper(neighboor), node),
                Reverse(OrderedFloat(neighboor.distance(to))),
            );
        }
    }

    None
}

fn build_path(
    node: Vec2Wrapper,
    from: Vec2,
    closed: HashMap<Vec2Wrapper, Vec2Wrapper>,
) -> Vec<Vec2> {
    let mut path = vec![*node];
    while path[path.len() - 1].distance(from) > 0.1 {
        path.push(*closed[&Vec2Wrapper(path[path.len() - 1])]);
    }

    path.into_iter().rev().collect()
}

fn neighboors(node: Vec2, rapier_ctx: &RapierContext) -> Vec<Vec2> {
    let neighboors = vec![
        node + Vec2::X,
        node - Vec2::X,
        node + Vec2::Y,
        node - Vec2::Y,
    ];
    neighboors
        .into_iter()
        .filter(|canditate| {
            let mut free = true;
            rapier_ctx.intersections_with_shape(
                *canditate,
                0.0,
                &Collider::cuboid(0.5, 0.5),
                QueryFilter::only_fixed(),
                |_| {
                    free = false;
                    false
                },
            );
            free
        })
        .collect()
}

#[derive(Deref, Clone, Copy, Debug)]
struct Vec2Wrapper(Vec2);

impl PartialEq for Vec2Wrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Vec2Wrapper {}

impl std::hash::Hash for Vec2Wrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        OrderedFloat(self.0.x).hash(state);
        OrderedFloat(self.0.y).hash(state);
    }
}
