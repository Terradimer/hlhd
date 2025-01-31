use bevy::app::{App, Plugin, Update};
use bevy_ecs::prelude::{IntoSystemConfigs, OnEnter};

use systems::*;

use crate::AppState;

mod components;
mod resources;
mod systems;

const JUMP_SRENGTH: f32 = 100.0;
const PLAYER_SPEED: f32 = 200.0;
const COYOTE_TIME: f32 = 0.2;

pub struct PlayerControllerPlugin;

// Todo: Add the remaining state transitions and their behavior
// Todo: Attach the sprite to the player and make a better system handling sprites

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                contact_detection_system,
                (movement_system, in_air, grounded, jumping),
            ),
            //.chain(),
        )
        .add_systems(OnEnter(AppState::Loaded), spawn_player);
    }
}
