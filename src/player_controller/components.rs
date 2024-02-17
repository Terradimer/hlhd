use bevy::prelude::Component;

#[derive(Component)]
pub struct Player; // Tag component for the player

#[derive(Component)]
pub struct InAirData {
    pub(crate) coyote_time: f32,
}