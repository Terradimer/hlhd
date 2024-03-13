use bevy::math::Vec2;
use bevy::prelude::{GamepadButtonType, KeyCode, MouseButton, Reflect, Resource};
use leafwing_input_manager::{
    axislike::VirtualAxis,
    input_map::InputMap,
    prelude::{InputKind, UserInput},
    Actionlike,
};

#[derive(Resource, Default)]
pub struct MousePosition {
    pub position: Vec2,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Inputs {
    Horizontal,
    Vertical,
    Jump,
    Shoot,
    Esc,
}

impl Inputs {
    fn variants() -> &'static [Inputs] {
        &[
            Self::Horizontal,
            Self::Vertical,
            Self::Jump,
            Self::Shoot,
            Self::Esc,
        ]
    }

    fn default_keyboard_input(&self) -> UserInput {
        match self {
            Self::Horizontal => UserInput::VirtualAxis(VirtualAxis::ad()),
            Self::Vertical => UserInput::VirtualAxis(VirtualAxis::ws()),
            Self::Jump => UserInput::Single(InputKind::PhysicalKey(KeyCode::Space)),
            Self::Shoot => UserInput::Single(InputKind::Mouse(MouseButton::Left)),
            Self::Esc => UserInput::Single(InputKind::PhysicalKey(KeyCode::Escape)),
        }
    }

    fn default_gamepad_input(&self) -> UserInput {
        match self {
            Self::Horizontal => UserInput::VirtualAxis(VirtualAxis::horizontal_dpad()),
            Self::Vertical => UserInput::VirtualAxis(VirtualAxis::vertical_dpad()),
            Self::Jump => UserInput::Single(InputKind::GamepadButton(GamepadButtonType::South)),
            Self::Shoot => {
                UserInput::Single(InputKind::GamepadButton(GamepadButtonType::RightTrigger2))
            }
            Self::Esc => UserInput::Single(InputKind::GamepadButton(GamepadButtonType::Start)),
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
