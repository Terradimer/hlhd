use bevy::prelude::*;
use leafwing_input_manager::axislike::VirtualAxis;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Inputs {
    Horizontal,
    Vertical,
    Jump,
}

impl Inputs {
    fn variants() -> &'static [Inputs] {
        &[Self::Horizontal, Self::Vertical, Self::Jump]
    }

    fn default_keyboard_input(&self) -> UserInput {
        match self {
            Self::Horizontal => UserInput::VirtualAxis(VirtualAxis::ad()),
            Self::Vertical => UserInput::VirtualAxis(VirtualAxis::ws()),
            Self::Jump => UserInput::Single(InputKind::PhysicalKey(KeyCode::Space)),
        }
    }

    fn default_gamepad_input(&self) -> UserInput {
        match self {
            Self::Horizontal => UserInput::VirtualAxis(VirtualAxis::horizontal_dpad()),
            Self::Vertical => UserInput::VirtualAxis(VirtualAxis::vertical_dpad()),
            Self::Jump => UserInput::Single(InputKind::GamepadButton(GamepadButtonType::South)),
        }
    }

    pub fn input_map() -> InputMap<Inputs> {
        let mut input_map = InputMap::default();

        for action in Inputs::variants() {
            input_map.insert(*action, Inputs::default_keyboard_input(action));
            input_map.insert(*action, Inputs::default_gamepad_input(action));
        }

        input_map
    }
}
