use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_ecs::prelude::IntoSystemConfigs;

use crate::camera::systems::update_cam_bounds;
use crate::world_generation::ui::WorldGenUIPlugin;
use crate::AppState;
use systems::*;

use self::events::{LoadRoomEvent, SaveRoomEvent};
pub(crate) mod components;
mod events;
mod functions;
mod systems;
mod ui;

pub const WINDOW_WIDTH: f32 = 1024.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;
const COLOR_PLATFORM: Color = Color::rgb(0.75, 0.75, 0.75);

const SNAP_SCALE: f32 = 10.;
const EDGE_THRESHOLD: f32 = 0.2;
const MIN_SCALE: f32 = 20.;

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (make_test_scene, update_cam_bounds).chain())
            .add_systems(
                Update,
                (
                    (save_room, load_room),
                    update_dev_entities.run_if(in_state(AppState::Dev)),
                    (
                        scale_dev_entities.run_if(in_state(AppState::Dev)),
                        dragging_env_entities.run_if(in_state(AppState::Dev)),
                    ),
                )
                    .chain(),
            )
            .add_plugins(WorldGenUIPlugin)
            .add_event::<SaveRoomEvent>()
            .add_event::<LoadRoomEvent>();
    }
}
