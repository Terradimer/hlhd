use bevy::prelude::{Resource, Vec2};

#[derive(Resource)]
pub struct CameraData {
    pub zoom: f32,
    pub offset: Vec2,
}
