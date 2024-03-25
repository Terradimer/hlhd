use bevy::prelude::Component;

#[derive(Component)]
pub struct MainCamera {
    pub(crate) default_scale: f32,
}

#[derive(Component)]
pub struct CamBoundsTracker;
