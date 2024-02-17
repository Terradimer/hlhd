use bevy::prelude::{Resource, States};
use bevy_ecs::schedule::ScheduleLabel;

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
}
