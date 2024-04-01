use bevy::prelude::{Component, Timer, TimerMode};

use super::COYOTE_TIME;

#[derive(Component)]
pub struct Player; // Tag component for the player

#[derive(Component, Default, Debug)]
pub struct ContactDirection {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub front: bool,
    pub back: bool,
}

// States
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct InAir;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded {
    pub coyote_time: Timer,
}

impl Default for Grounded {
    fn default() -> Self {
        Grounded {
            coyote_time: Timer::from_seconds(COYOTE_TIME, TimerMode::Once),
        }
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Jumping;
