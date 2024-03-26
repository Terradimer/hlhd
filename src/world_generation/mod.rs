use bevy::{
    app::{App, Plugin},
    prelude::*,
};
use bevy_ecs::prelude::IntoSystemConfigs;

use crate::{world_generation::ui::WorldGenUIPlugin, AppState};
use systems::*;

use self::rooms::{systems::update_room_bounds, RoomsPlugin};

pub(crate) mod components;
mod functions;
pub(crate) mod rooms;
mod systems;
mod ui;

pub const WINDOW_WIDTH: f32 = 1024.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
const COLOR_PLATFORM: Color = Color::rgb(0.75, 0.75, 0.75);

const SNAP_SCALE: f32 = 10.;
const EDGE_THRESHOLD: f32 = 0.2;
const MIN_SCALE: f32 = 20.;

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(AppState::Loading),
            (start_scene, rooms::systems::load_room).chain(),
        )
        .add_systems(
            Update,
            (
                update_dev_entities.run_if(in_state(AppState::Dev)),
                (
                    scale_dev_entities.run_if(in_state(AppState::Dev)),
                    dragging_env_entities.run_if(in_state(AppState::Dev)),
                ),
            )
                .chain(),
        )
        .add_plugins((WorldGenUIPlugin, RoomsPlugin));
    }
}
