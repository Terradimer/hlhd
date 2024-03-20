use crate::AppState;
use bevy::app::{App, Plugin};
use bevy::prelude::Update;
use bevy_ecs::prelude::{in_state, IntoSystemConfigs};
use leafwing_input_manager::prelude::{ActionState, InputManagerPlugin};
use systems::*;

pub mod resources;
mod systems;

pub struct InputHandlerPlugin;

impl Plugin for InputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<resources::Inputs>::default())
            .init_resource::<ActionState<resources::Inputs>>()
            .init_resource::<resources::MousePosition>()
            .insert_resource(resources::Inputs::input_map())
            .add_systems(
                Update,
                (
                    update_cursor_position,
                    enter_playing.run_if(in_state(AppState::Dev)),
                    enter_dev.run_if(in_state(AppState::Playing)),
                ),
            );
    }
}
