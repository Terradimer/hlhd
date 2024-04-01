use crate::macros::query_guard;
use crate::AppState;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use leafwing_input_manager::action_state::ActionState;

use crate::camera::components::MainCamera;
use crate::input::resources::MousePosition;

use super::Inputs;

pub fn update_cursor_position(
    mut mouse_coords: ResMut<MousePosition>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = query_guard!(q_camera.get_single());
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse_coords.position = world_position;
    }
}


pub fn enter_editor(
    input: Res<ActionState<Inputs>>, 
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut state: ResMut<NextState<AppState>>
) {
    if input.just_pressed(&Inputs::Esc) {
        let mut primary_window = q_windows.single_mut();
        primary_window.cursor.grab_mode = CursorGrabMode::None;
        primary_window.cursor.visible = true;
        state.set(AppState::Editor);
    }
}

pub fn exit_editor(
    input: Res<ActionState<Inputs>>, 
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut state: ResMut<NextState<AppState>>
) {
    if input.just_pressed(&Inputs::Esc) {
        let mut primary_window = q_windows.single_mut();
        primary_window.cursor.grab_mode = CursorGrabMode::Locked;
        primary_window.cursor.visible = false;
        state.set(AppState::Playing);
    }
}