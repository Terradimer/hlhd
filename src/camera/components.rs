use bevy::prelude::{Component, Vec3};

#[derive(Component)]
pub struct MainCamera {
    pub(crate) default_scale: f32,
}

#[derive(Component)]
pub struct CamBoundsTracker;
