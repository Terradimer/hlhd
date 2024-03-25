use bevy::{log::tracing_subscriber::field::debug, math::vec2, prelude::*};

#[derive(Resource, Debug)]
pub struct CamBounds {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

impl Default for CamBounds {
    fn default() -> Self {
        Self {
            min_x: f32::MAX,
            max_x: f32::MIN,
            min_y: f32::MAX,
            max_y: f32::MIN,
        }
    }
}
