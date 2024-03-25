use bevy::app::{App, Plugin, Update};
use bevy_ecs::prelude::{in_state, IntoSystemConfigs, OnExit};

use systems::*;

use crate::AppState;

pub mod components;
mod functions;
mod systems;

const JUMP_STRENGTH: f32 = 100.0;
const PLAYER_SPEED: f32 = 200.0;
const COYOTE_TIME: f32 = 0.2;

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                contact_detection,
                (movement_system, in_air, grounded, jumping),
            )
                .run_if(in_state(AppState::Playing)),
            //.chain(),
        )
        .add_systems(OnExit(AppState::Loading), spawn_player);
    }
}
