use bevy::{
    prelude::*,
    render::{settings::RenderCreation, *},
    window::*,
};
// External Crates
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::input::resources::Inputs;

mod animation;
mod collision_groups;

mod input;
mod macros;
mod player_controller;
mod time;
mod world_generation;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Playing,
    Dev,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "HardLight HyperDriver".to_string(),
                        resolution: WindowResolution::new(
                            world_generation::WINDOW_WIDTH,
                            world_generation::WINDOW_HEIGHT,
                        ),
                        present_mode: PresentMode::AutoNoVsync,
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(settings::WgpuSettings {
                        backends: Some(settings::Backends::VULKAN),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), // this is just for the pixel art demo sprites
            //FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
            RapierDebugRenderPlugin::default(),
            time::TimeScalarPlugin,
            input::InputHandlerPlugin,
            animation::AnimationPlugin,
            player_controller::PlayerControllerPlugin,
            world_generation::WorldGenerationPlugin,
        ))
        .init_state::<AppState>()
        .run();
}
