use bevy::math::Vec2;
use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct MousePosition {
    pub position: Vec2,
}
