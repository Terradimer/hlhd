use bevy::prelude::Component;

#[derive(Component)]
pub struct MainCamera {
    pub(crate) default_scale: f32,
}
