use bevy::{
    diagnostic::LogDiagnosticsPlugin,
    prelude::*,
    render::{settings::RenderCreation, *},
    window::*,
};

use crate::{
    animation::AnimationHandlerPlugin, camera::CameraHandlerPlugin, input::InputHandlerPlugin,
    player_controller::PlayerControllerPlugin, time::TimeScalarPlugin,
    world_generation::WorldGenerationPlugin,
};
use bevy_rapier2d::prelude::*;

mod animation;
mod camera;
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
            RapierDebugRenderPlugin::disabled(Default::default()),
            // Internal Crates
            TimeScalarPlugin,
            InputHandlerPlugin,
            AnimationHandlerPlugin,
            PlayerControllerPlugin,
            WorldGenerationPlugin,
            CameraHandlerPlugin,
        ))
        .insert_resource(DebugRenderContext {
            enabled: false,
            ..default()
        })
        .init_state::<AppState>()
        .run();
}
