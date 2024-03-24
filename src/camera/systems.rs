use bevy::prelude::*;
use bevy_pancam::PanCam;
use crate::camera::components::MainCamera;
use bevy_rapier2d::render::DebugRenderContext;
use crate::camera::{ROOM_MAX, ROOM_MIN};
use crate::macros::query_guard;
use crate::player_controller::components::Player;
use crate::time::resources::ScaledTime;

pub fn on_enter_playing(
    mut q_camera: Query<(&mut OrthographicProjection, &mut PanCam, &MainCamera)>,
    mut debug_rendering: ResMut<DebugRenderContext>,
) {
    debug_rendering.enabled = false;

    if let Ok((mut cam, mut pan, cam_data)) = q_camera.get_single_mut() {
        pan.enabled = false;
        cam.scale = cam_data.default_scale;
    }
}

pub fn on_enter_dev(
    mut q_camera: Query<&mut PanCam, With<MainCamera>>,
    mut debug_rendering: ResMut<DebugRenderContext>
) {
    debug_rendering.enabled = true;
    if let Ok(mut cam) = q_camera.get_single_mut() {
        cam.enabled = true;
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera {
            default_scale: 1.
        },
        PanCam {
            grab_buttons: vec![MouseButton::Right],
            enabled: false,
            zoom_to_cursor: true,
            min_scale: 1.,
            max_scale: Some(40.),
            ..default()
        }
    ));
}

pub fn follow_player(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    time: Res<ScaledTime>
) {
    let (player_transform, mut camera_transform) =
        query_guard!(player_query.get_single(), camera_query.get_single_mut());

    let target_position = camera_transform.translation.lerp(player_transform.translation, time.delta.as_secs_f32());
    camera_transform.translation.x = target_position.x.clamp(ROOM_MIN.x, ROOM_MAX.x);
    camera_transform.translation.y = target_position.y.clamp(ROOM_MIN.y, ROOM_MAX.y);
}