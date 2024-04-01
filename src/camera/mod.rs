use crate::AppState;

use self::systems::*;
use bevy::app::*;
use bevy_ecs::schedule::{common_conditions::in_state, IntoSystemConfigs};

pub mod components;
pub mod systems;

pub struct CameraHandlerPlugin;

impl Plugin for CameraHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, orbit_camera.run_if(in_state(AppState::Playing)));
        // .add_systems(Update, follow_player.run_if(in_state(AppState::Playing)))
        // .add_systems(OnEnter(AppState::Dev), on_enter_dev)
        // .add_systems(OnEnter(AppState::Playing), on_enter_playing);
    }
}
