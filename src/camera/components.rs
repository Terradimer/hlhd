use bevy::prelude::Component;

#[derive(Component)]
pub struct MainCamera {
    pub offset: (f32, f32),
    pub zoom: f32
}
