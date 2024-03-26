mod components;
mod functions;
mod systems;

use crate::AppState;
use bevy::app::{App, Plugin, Update};
use bevy_ecs::prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit};
use systems::*;

pub struct WorldGenUIPlugin;

impl Plugin for WorldGenUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Dev), setup_buttons)
            .add_systems(OnExit(AppState::Dev), cleanup_dev_buttons)
            .add_systems(Update, on_click.run_if(in_state(AppState::Dev)));
    }
}
