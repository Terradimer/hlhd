use bevy::prelude::*;
use leafwing_input_manager::{
    axislike::{DualAxis, VirtualAxis, VirtualDPad},
    input_map::{self, InputMap},
    prelude::{ActionState, InputManagerPlugin},
    user_input::{InputKind, UserInput},
    Actionlike,
};
use systems::*;

use crate::AppState;

pub mod resources;
mod systems;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Inputs {
    Movement,
    Jump,
    Shoot,
    Esc,
}

impl Inputs {
    pub fn input_map() -> InputMap<Inputs> {
        let mut input_map = InputMap::default();

        input_map.insert(Self::Movement, VirtualDPad::wasd());
        input_map.insert(Self::Jump, KeyCode::Space);
        input_map.insert(Self::Shoot, MouseButton::Left);
        input_map.insert(Self::Esc, KeyCode::Escape);

        input_map.insert(Self::Movement, DualAxis::left_stick());
        input_map.insert(Self::Jump, GamepadButtonType::South);
        input_map.insert(Self::Shoot, GamepadButtonType::RightTrigger2);
        input_map.insert(Self::Esc, GamepadButtonType::Start);

        input_map
    }
}

pub struct InputHandlerPlugin;

impl Plugin for InputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Inputs>::default())
            .init_resource::<ActionState<Inputs>>()
            .init_resource::<resources::MousePosition>()
            .insert_resource(Inputs::input_map())
            .add_systems(
                Update,
                (
                    update_cursor_position,
                    enter_editor.run_if(in_state(AppState::Playing)),
                    exit_editor.run_if(in_state(AppState::Editor)),
                ),
            );
    }
}
