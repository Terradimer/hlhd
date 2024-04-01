use bevy::{
    diagnostic::LogDiagnosticsPlugin,
    input::keyboard::{self, KeyboardInput},
    prelude::*,
    render::{settings::RenderCreation, *},
    window::*,
};

use bevy_editor_pls::{
    controls::{self, EditorControls},
    EditorPlugin,
};
// use crate::{
//     animation::AnimationHandlerPlugin, camera::CameraHandlerPlugin, input::InputHandlerPlugin,
//     player_controller::PlayerControllerPlugin, time::TimeScalarPlugin,
//     world_generation::WorldGenerationPlugin,
// };
use bevy_rapier3d::prelude::*;
use camera::CameraHandlerPlugin;
use input::InputHandlerPlugin;
use player::PlayerPlugin;
use time::TimeScalarPlugin;
use user_settings::{SettingsPlugin, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};
use world_gen::WorldGenerationPlugin;

mod camera;
mod collision_groups;
mod input;
mod macros;
// mod world_generation;
mod player;
mod state_machines;
mod time;
mod user_settings;
mod world_gen;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Playing,
    Editor,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "HardLight HyperDriver".to_string(),
                        resolution: WindowResolution::new(
                            DEFAULT_WINDOW_WIDTH,
                            DEFAULT_WINDOW_HEIGHT,
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
            RapierPhysicsPlugin::<NoUserData>::default(),
            // Internal Crates
            SettingsPlugin,
            TimeScalarPlugin,
            EditorPlugin::default(),
            InputHandlerPlugin,
            PlayerPlugin,
            WorldGenerationPlugin,
            CameraHandlerPlugin,
        ))
        .insert_resource(DebugRenderContext {
            enabled: false,
            ..default()
        })
        .insert_resource(editor_controls())
        .init_state::<AppState>()
        .run();
}

fn editor_controls() -> EditorControls {
    let mut editor_controls = EditorControls::default_bindings();
    editor_controls.unbind(controls::Action::PlayPauseEditor);

    editor_controls.insert(
        controls::Action::PlayPauseEditor,
        controls::Binding {
            input: controls::UserInput::Single(controls::Button::Keyboard(KeyCode::Escape)),
            conditions: vec![controls::BindingCondition::ListeningForText(false)],
        },
    );

    editor_controls
}
