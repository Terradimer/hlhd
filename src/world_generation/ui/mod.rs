mod components;
mod functions;
mod systems;

use crate::AppState;
use bevy::app::{App, Plugin, Update};
use bevy_ecs::prelude::{in_state, IntoSystemConfigs};
use bevy_egui::EguiPlugin;
use systems::*;

pub struct WorldGenUIPlugin;

impl Plugin for WorldGenUIPlugin {
    fn build(&self, app: &mut App) {
        app
        // .add_systems(OnEnter(AppState::Dev), setup_dev_button)
        // .add_systems(OnExit(AppState::Dev), cleanup_dev_button)
        // .add_systems(Update, save_level_on_click.run_if(in_state(AppState::Dev)));
        .add_plugins(EguiPlugin)
        .add_systems(Update, egui_ui_test.run_if(in_state(AppState::Dev)));
    }
}
