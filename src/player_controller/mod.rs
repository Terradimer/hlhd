use crate::player_controller::resources::PlayerState;
use bevy::app::{App, Plugin, Startup, Update};
use bevy_ecs::prelude::{in_state, IntoSystemConfigs};

mod components;
mod resources;
mod systems;

const JUMP_VELOCITY: f32 = 500.0;
const PLAYER_SPEED: f32 = 200.0;

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlayerState>()
            .insert_resource(resources::PreviousState {
                state: Some(resources::PlayerState::default()),
            })
            .add_systems(Startup, systems::spawn_player)
            .add_systems(
                Update,
                (systems::movement_system).run_if(in_state(PlayerState::InAir)),
            );
    }
}
