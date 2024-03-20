use crate::world_generation::components::Edges;
use crate::world_generation::EDGE_THRESHOLD;
use bevy::prelude::*;

pub fn detect_edge(transform: &Transform, mouse_position: Vec2) -> Option<Edges> {
    let edge_threshold_x = (transform.scale.x * EDGE_THRESHOLD).min(20.);
    let edge_threshold_y = (transform.scale.y * EDGE_THRESHOLD).min(20.);

    let min_x = transform.translation.x - transform.scale.x / 2.0;
    let max_x = transform.translation.x + transform.scale.x / 2.0;
    let min_y = transform.translation.y - transform.scale.y / 2.0;
    let max_y = transform.translation.y + transform.scale.y / 2.0;

    let edges = Edges {
        vertical: (mouse_position.x > max_x - edge_threshold_x && mouse_position.x < max_x + edge_threshold_x)
            as i8
            - (mouse_position.x < min_x + edge_threshold_x && mouse_position.x > min_x - edge_threshold_x)
            as i8,
        horizontal: (mouse_position.y > max_y - edge_threshold_y && mouse_position.y < max_y + edge_threshold_y)
            as i8
            - (mouse_position.y < min_y + edge_threshold_y && mouse_position.y > min_y - edge_threshold_y)
            as i8,
    };


    if edges.horizontal != 0 || edges.vertical != 0 {
        Some(edges)
    } else {
        None
    }
}
