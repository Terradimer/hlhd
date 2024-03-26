use super::{
    rooms::components::{RoomBoundsAffector, Saveable},
    COLOR_PLATFORM,
};
use crate::world_generation::{
    components::{Edges, *},
    EDGE_THRESHOLD,
};
use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::RigidBody,
    geometry::{Collider, CollisionGroups, Friction, Sensor},
};

pub fn detect_edge(transform: &Transform, mouse_position: Vec2) -> Option<Edges> {
    let edge_threshold_x = (transform.scale.x * EDGE_THRESHOLD).min(20.);
    let edge_threshold_y = (transform.scale.y * EDGE_THRESHOLD).min(20.);

    let min_x = transform.translation.x - transform.scale.x / 2.0;
    let max_x = transform.translation.x + transform.scale.x / 2.0;
    let min_y = transform.translation.y - transform.scale.y / 2.0;
    let max_y = transform.translation.y + transform.scale.y / 2.0;

    let edges = Edges {
        vertical: (mouse_position.x > max_x - edge_threshold_x
            && mouse_position.x < max_x + edge_threshold_x) as i8
            - (mouse_position.x < min_x + edge_threshold_x
                && mouse_position.x > min_x - edge_threshold_x) as i8,
        horizontal: (mouse_position.y > max_y - edge_threshold_y
            && mouse_position.y < max_y + edge_threshold_y) as i8
            - (mouse_position.y < min_y + edge_threshold_y
                && mouse_position.y > min_y - edge_threshold_y) as i8,
    };

    if edges.horizontal != 0 || edges.vertical != 0 {
        Some(edges)
    } else {
        None
    }
}

pub fn gen_platform(
    translation: Vec3,
    scale: Vec3,
) -> (
    SpriteBundle,
    Scalable,
    Saveable,
    Draggable,
    RigidBody,
    Collider,
    RoomBoundsAffector,
    CollisionGroups,
    Friction,
) {
    (
        SpriteBundle {
            sprite: Sprite {
                color: COLOR_PLATFORM,
                ..Default::default()
            },
            transform: Transform {
                translation,
                scale,
                ..Default::default()
            },
            ..Default::default()
        },
        Scalable,
        Saveable::Platform,
        Draggable,
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5),
        RoomBoundsAffector,
        crate::collision_groups::Groups::environment(),
        Friction::new(0.),
    )
}

pub fn gen_exit(
    translation: Vec3,
    scale: Vec3,
) -> (
    SpriteBundle,
    Scalable,
    Saveable,
    Draggable,
    Collider,
    Sensor,
    CollisionGroups,
) {
    (
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK.with_a(0.),
                ..Default::default()
            },
            transform: Transform {
                translation,
                scale,
                ..Default::default()
            },
            ..Default::default()
        },
        Scalable,
        Saveable::Exit,
        Draggable,
        Collider::cuboid(0.5, 0.5),
        Sensor,
        crate::collision_groups::Groups::environment(),
    )
}
