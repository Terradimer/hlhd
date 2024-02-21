use bevy::prelude::Component;

use crate::animation::components::*;

#[derive(Component)]
pub struct Player; // Tag component for the player

#[derive(Component, Default, Debug)]
pub struct ContactDirection {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Component)]
pub struct PlayerIndexMap {
    pub idle: Animation,
    pub falling: Animation,
    pub walk: Animation,
}

// States
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct InAirState {
    pub(crate) coyote_time: f32,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct GroundedState;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct JumpingState;
