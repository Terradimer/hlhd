use bevy::prelude::Component;

#[derive(Component)]
pub struct MainCamera {
    pub zoom: f32,
}

#[derive(Component)]
pub struct CameraAnchor;
