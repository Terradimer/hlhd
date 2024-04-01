use bevy::app::{App, Plugin, Startup, Update};
use bevy_ecs::prelude::{in_state, IntoSystemConfigs, OnExit};

use systems::*;

pub mod components;
mod systems;

const JUMP_STRENGTH: f32 = 200.0;
pub const PLAYER_SPEED: f32 = 20000.0;
const COYOTE_TIME: f32 = 0.2;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                contact_detection,
                (jumping, in_air, grounded, movement_system)
                //(movement_system, in_air, grounded, jumping),
            )
                //.run_if(in_state(AppState::Playing)),
            //.chain(),
        )
        .add_systems(Startup, spawn_player);
    }
}
