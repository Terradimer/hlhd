use bevy::prelude::Resource;

#[derive(Resource)]
pub struct CameraConfig {
    pub sensitivity: f32,
}
