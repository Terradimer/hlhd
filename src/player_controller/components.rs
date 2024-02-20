use bevy::prelude::Component;
use bevy::utils::HashMap;

use crate::animation::components::Animation;
use crate::player_controller::resources::PlayerState;

#[derive(Component)]
pub struct Player; // Tag component for the player

#[derive(Component)]
pub struct PlayerSprite; // Tag component for the player sprite

#[derive(Component, Default, Debug)]
pub struct ContactDirection {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct InAirData {
    pub(crate) coyote_time: f32,
}

#[derive(Component)]
pub struct PlayerAnimator {
    pub animations: HashMap<PlayerState, Animation>,
}
