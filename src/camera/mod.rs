use self::{resources::*, systems::*};
use crate::AppState;
use bevy::app::*;
use bevy::prelude::*;
use bevy_pancam::PanCamPlugin;

pub mod components;
pub mod resources;
pub mod systems;

pub struct CameraHandlerPlugin;

impl Plugin for CameraHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin::default())
            .insert_resource(CamBounds::default())
            .add_systems(Startup, setup)
            .add_systems(Update, follow_player.run_if(in_state(AppState::Playing)))
            .add_systems(OnEnter(AppState::Dev), on_enter_dev)
            .add_systems(
                OnEnter(AppState::Playing),
                (update_cam_bounds, on_enter_playing).chain(),
            );
    }
}
