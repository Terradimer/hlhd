use bevy::prelude::{Resource, States};

#[derive(Resource)]
pub struct PreviousState {
    pub state: Option<PlayerState>,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, States)] // Necessary for Bevy's State
pub enum PlayerState {
    Grounded,
    #[default]
    InAir,
    Jumping,
}
