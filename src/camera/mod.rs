use bevy::app::*;
use bevy::prelude::*;
use bevy_ecs::prelude::*;
use bevy_pancam::PanCamPlugin;
use crate::AppState;
use crate::camera::systems::*;

pub mod systems;
pub mod components;

pub struct CameraHandlerPlugin;

// temp while I finish room development
pub const ROOM_MAX: Vec2 = Vec2::new(100.0, 25.0);
pub const ROOM_MIN: Vec2 = Vec2::new(-100.0, -25.0);

impl Plugin for CameraHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PanCamPlugin::default())
            .add_systems(Startup, setup)
            .add_systems(Update, follow_player.run_if(in_state(AppState::Playing)))
            .add_systems(OnEnter(AppState::Dev), on_enter_dev)
            .add_systems(OnEnter(AppState::Playing), on_enter_playing);
    }
}