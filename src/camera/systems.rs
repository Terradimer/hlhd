use std::f32::consts::PI;

use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_rapier3d::{pipeline::QueryFilter, plugin::RapierContext};

use crate::{
    collision_groups::Groups, macros::query_guard, player::components::Player, time,
    user_settings::resources::CameraConfig,
};

use super::{components::*, resources::CameraData};

pub fn setup(mut q_windows: Query<&mut Window, With<PrimaryWindow>>, mut commands: Commands) {
    let mut primary_window = q_windows.single_mut();

    commands.spawn((Camera3dBundle::default(), MainCamera { zoom: 25. }));
    commands.spawn((TransformBundle::default(), CameraAnchor));

    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor.visible = false;
}

pub fn update_anchor(
    mut q_anchor: Query<&mut Transform, With<CameraAnchor>>,
    q_player: Query<&Transform, (With<Player>, Without<CameraAnchor>)>,
    cam_data: Res<CameraData>,
) {
    let (mut anchor_transform, player_transform) =
        query_guard!(q_anchor.get_single_mut(), q_player.get_single());

    anchor_transform.translation = player_transform.translation + cam_data.offset.extend(0.);
}

pub fn orbit_camera(
    mut q_anchor: Query<&mut Transform, (With<CameraAnchor>, Without<MainCamera>)>,
    mut q_camera: Query<(&mut Transform, &MainCamera)>,
    mut mouse_motion: EventReader<MouseMotion>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    camera_config: Res<CameraConfig>,
    context: Res<RapierContext>,
) {
    let (mut anchor_transform, window, (mut cam_transform, cam)) = query_guard!(
        q_anchor.get_single_mut(),
        q_window.get_single(),
        q_camera.get_single_mut()
    );

    let mut delta = Vec2::ZERO;
    for event in mouse_motion.read() {
        delta = event.delta;
    }

    if delta.length_squared() > 0.0 {
        let delta_x = {
            let delta = delta.x / window.width() * std::f32::consts::PI * camera_config.sensitivity;
            delta
        };

        let delta_y = delta.y / window.height() * PI * camera_config.sensitivity;
        let yaw = Quat::from_rotation_y(-delta_x);
        let pitch = Quat::from_rotation_x(-delta_y);
        anchor_transform.rotation = yaw * anchor_transform.rotation;

        let new_rotation = anchor_transform.rotation * pitch;

        let up_vector = new_rotation * Vec3::Y;
        if up_vector.y > 0.0 {
            anchor_transform.rotation = new_rotation;
        }
    }

    let direction = anchor_transform.back().xyz();

    if let Some((entity, toi)) = context.cast_ray(
        anchor_transform.translation,
        direction,
        cam.zoom,
        true,
        QueryFilter {
            groups: Some(Groups::player()),
            ..default()
        },
    ) {
        cam_transform.translation = anchor_transform.translation - direction + toi * direction;
    } else {
        cam_transform.translation = anchor_transform.translation - direction + cam.zoom * direction;
    }

    cam_transform.look_at(anchor_transform.translation, Vec3::Y);
}
