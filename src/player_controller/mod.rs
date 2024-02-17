mod components;
mod resources;
mod systems;

use bevy::app::{App, Plugin, Startup, Update};
use bevy_ecs::prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit};
use resources::PlayerState;

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
            .add_systems(OnExit(PlayerState::InAir), systems::exit_in_air)
            .add_systems(OnEnter(PlayerState::InAir), systems::enter_in_air)
            .add_systems(
                Update,
                (
                    systems::movement_system.run_if(in_state(PlayerState::Grounded)),
                    systems::movement_system.run_if(in_state(PlayerState::InAir)),
                ),
            )
            .add_systems(Startup, systems::spawn_player);
    }
}
