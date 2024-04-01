use crate::AppState;

use self::{resources::CameraData, systems::*};
use bevy::prelude::*;
use bevy_ecs::schedule::{common_conditions::in_state, IntoSystemConfigs};

pub mod components;
pub mod resources;
pub mod systems;

pub struct CameraHandlerPlugin;

impl Plugin for CameraHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraData {
            zoom: 20.,
            offset: Vec2::ZERO,
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (update_anchor.before(orbit_camera), orbit_camera).run_if(in_state(AppState::Playing)),
        );
        // .add_systems(Update, follow_player.run_if(in_state(AppState::Playing)))
        // .add_systems(OnEnter(AppState::Dev), on_enter_dev)
        // .add_systems(OnEnter(AppState::Playing), on_enter_playing);
    }
}
