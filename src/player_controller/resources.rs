use bevy::prelude::{Resource, States};

#[derive(Resource)]
pub struct PreviousState {
    pub state: Option<PlayerState>,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, States)]
pub enum PlayerState {
    Grounded,
    #[default]
    InAir,
    Jumping,
    Idle,
}
