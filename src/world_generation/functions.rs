use super::COLOR_PLATFORM;
use crate::{
    camera::components::CamBoundsTracker,
    world_generation::{
        components::{Edges, *},
        EDGE_THRESHOLD,
    },
};
use bevy::prelude::*;
use bevy_rapier2d::{
    dynamics::RigidBody,
    geometry::{Collider, CollisionGroups, Friction},
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
    _translation: Vec3,
    _scale: Vec3,
) -> (
    SpriteBundle,
    Scalable,
    Saveable,
    Draggable,
    RigidBody,
    Collider,
    CamBoundsTracker,
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
                translation: _translation,
                scale: _scale,
                ..Default::default()
            },
            ..Default::default()
        },
        Scalable,
        Saveable,
        Draggable,
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5),
        CamBoundsTracker,
        crate::collision_groups::Groups::environment(),
        Friction::new(0.),
    )
}
