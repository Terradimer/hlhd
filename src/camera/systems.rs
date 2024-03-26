use crate::{
    macros::query_guard, player_controller::components::Player, time::resources::ScaledTime,
    world_generation::rooms::resources::CurrentRoom,
};
use bevy::prelude::*;
use bevy_pancam::PanCam;
use bevy_rapier2d::render::DebugRenderContext;

use super::components::*;

pub fn on_enter_playing(
    mut q_camera: Query<(
        &mut OrthographicProjection,
        &mut Transform,
        &mut PanCam,
        &MainCamera,
    )>,
    mut debug_rendering: ResMut<DebugRenderContext>,
    q_player: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    debug_rendering.enabled = false;

    let ((mut cam, mut cam_transform, mut pan, cam_data), player) =
        query_guard!(q_camera.get_single_mut(), q_player.get_single());

    pan.enabled = false;
    cam.scale = cam_data.default_scale;

    cam_transform.translation = player.translation;
}

pub fn on_enter_dev(
    mut q_camera: Query<&mut PanCam, With<MainCamera>>,
    mut debug_rendering: ResMut<DebugRenderContext>,
) {
    debug_rendering.enabled = true;
    if let Ok(mut cam) = q_camera.get_single_mut() {
        cam.enabled = true;
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera { default_scale: 1. },
        PanCam {
            grab_buttons: vec![MouseButton::Right],
            enabled: false,
            zoom_to_cursor: true,
            min_scale: 1.,
            max_scale: Some(40.),
            ..default()
        },
    ));
}

pub fn follow_player(
    mut camera_query: Query<(&mut Transform, &OrthographicProjection), With<MainCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    current_room: Res<CurrentRoom>,
    time: Res<ScaledTime>,
) {
    let bounds = &current_room.room.bounds;
    let (player_transform, (mut camera_transform, ortho)) =
        query_guard!(player_query.get_single(), camera_query.get_single_mut());

    let ortho_projection_height = ortho.area.height() / 2.;
    let ortho_projection_width = ortho.area.width() / 2.;

    let target_position = camera_transform
        .translation
        .lerp(player_transform.translation, time.delta.as_secs_f32());

    if target_position.x < bounds.min_x + ortho_projection_width {
        camera_transform.translation.x = bounds.min_x + ortho_projection_width;
    } else if target_position.x > bounds.max_x - ortho_projection_width {
        camera_transform.translation.x = bounds.max_x - ortho_projection_width;
    } else {
        camera_transform.translation.x = target_position.x;
    }

    if target_position.y < bounds.min_y + ortho_projection_height {
        camera_transform.translation.y = bounds.min_y + ortho_projection_height;
    } else if target_position.y > bounds.max_y - ortho_projection_height {
        camera_transform.translation.y = bounds.max_y - ortho_projection_height;
    } else {
        camera_transform.translation.y = target_position.y;
    }
}
