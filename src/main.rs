// Local Files
mod collision_groups;
mod input_handler;
mod player_controller;
mod world_gen;

use input_handler::Inputs;

// External Crates
use bevy::{
    prelude::*,
    render::{settings::RenderCreation, *},
    window::*,
};
use bevy_ecs::schedule::NodeId::Set;
use bevy_ecs::schedule::SystemSet;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "HardLight HyperDriver".to_string(),
                        resolution: WindowResolution::new(
                            world_gen::WINDOW_WIDTH,
                            world_gen::WINDOW_HEIGHT,
                        ),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(settings::WgpuSettings {
                        backends: Some(settings::Backends::DX12),
                        ..default()
                    }),
                })
                .set(ImagePlugin::default_nearest()), // this is just for the pixel art demo sprites
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RapierDebugRenderPlugin::default(),
            InputManagerPlugin::<Inputs>::default(),
            player_controller::PlayerControllerPlugin
        ))
        .init_resource::<ActionState<Inputs>>()
        .insert_resource(Inputs::input_map())
        .add_systems(
            Startup,
            (world_gen::make_test_scene),
        )

        //.add_systems(Update, (movement_system.in)
        .run();
}