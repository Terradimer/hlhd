use crate::AppState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use leafwing_input_manager::action_state::ActionState;

use crate::camera::components::MainCamera;
use crate::input::resources::{Inputs, MousePosition};
use crate::time::resources::ScaledTime;

pub fn update_cursor_position(
    mut mouse_coords: ResMut<MousePosition>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse_coords.position = world_position;
    }
}

pub fn enter_playing(
    input: Res<ActionState<Inputs>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut time: ResMut<ScaledTime>,
) {
    if input.just_pressed(&Inputs::Esc) {
        time.scale = time.stored_scale;
        next_state.set(AppState::Playing)
    }
}

pub fn enter_dev(
    input: Res<ActionState<Inputs>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut time: ResMut<ScaledTime>,
) {
    if input.just_pressed(&Inputs::Esc) {
        time.stored_scale = time.scale;
        time.scale = 0.;
        next_state.set(AppState::Dev)
    }
}
