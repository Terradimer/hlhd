use bevy::app::{App, Plugin, Update};
use bevy_ecs::prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit};

use resources::PlayerState;
use systems::*;

use crate::AppState;

mod components;
mod resources;
mod systems;

const JUMP_SRENGTH: f32 = 500.0;
const PLAYER_SPEED: f32 = 200.0;

pub struct PlayerControllerPlugin;

// Todo: Add the remaining state transitions and their behavior
// Todo: Attach the sprite to the player and make a better system handling sprites

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlayerState>()
            .insert_resource(resources::PreviousState {
                state: Some(PlayerState::default()),
            })
            // Heads up we might want to make this sequenced and state dependant
            .add_systems(
                Update,
                (
                    contact_detection_system, // 1st
                    (
                        movement_system.run_if(in_state(PlayerState::InAir)),
                        update_in_air.run_if(in_state(PlayerState::InAir)),
                        movement_system.run_if(in_state(PlayerState::Grounded)),
                        update_grounded.run_if(in_state(PlayerState::Grounded))
                    ), // 2nd any order
                ),
                //.chain(),
            )
            .add_systems(OnExit(PlayerState::InAir), exit_in_air)
            .add_systems(OnEnter(PlayerState::InAir), enter_in_air)
            .add_systems(OnEnter(PlayerState::Grounded), enter_grounded)
            .add_systems(OnExit(PlayerState::Grounded), exit_grounded)
            .add_systems(OnEnter(AppState::Loaded), spawn_player);
    }
}
