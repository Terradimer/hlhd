use std::f32::consts::PI;

use bevy::{input::mouse::MouseMotion, prelude::*, window::{CursorGrabMode, PrimaryWindow}};
use bevy_rapier3d::{pipeline::QueryFilter, plugin::RapierContext};

use crate::{collision_groups::Groups, player::components::Player, time, user_settings::MOUSE_SENSITIVITY};

use super::components::*;

pub fn setup(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>, 
    mut commands: Commands, 
) {
    let mut primary_window = q_windows.single_mut();

    commands.spawn((
        Camera3dBundle::default(),
        MainCamera {
            offset: (0., 2.5),
            zoom: 25.
        },
    ));
    
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor.visible = false;
}

pub fn orbit_camera(
    window_q: Query<&Window, With<PrimaryWindow>>,
    mut q_camera: Query<(&mut Transform, &MainCamera)>,
    mut mouse_evr: EventReader<MouseMotion>,
    q_player: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    context: Res<RapierContext>,
    time: Res<Time>
) {
    let mut rotation = Vec2::ZERO;
    for ev in mouse_evr.read() {
        rotation = ev.delta;
    }

    let Ok((mut cam_transform, camera)) = q_camera.get_single_mut() else {
        return;
    };

    let Ok(player_transform) = q_player.get_single() else {
        return;
    };

    if rotation.length_squared() > 0.0 {
        let window = window_q.get_single().unwrap();
        let delta_x = {
            let delta = rotation.x / window.width() * std::f32::consts::PI * MOUSE_SENSITIVITY;
            delta
        };

        let delta_y = rotation.y / window.height() * PI * MOUSE_SENSITIVITY;
        let yaw = Quat::from_rotation_y(-delta_x);
        let pitch = Quat::from_rotation_x(-delta_y);
        cam_transform.rotation = yaw * cam_transform.rotation; 

        let new_rotation = cam_transform.rotation * pitch;

        let up_vector = new_rotation * Vec3::Y;
        if up_vector.y > 0.0 {
            cam_transform.rotation = new_rotation;
        }
    }

    let rot_matrix = Mat3::from_quat(cam_transform.rotation);
    let offset = rot_matrix.mul_vec3(Vec3::new(camera.offset.0, camera.offset.1, 0.0));

    let target_pos = rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, camera.zoom)) + offset;

    let ray_direction = (target_pos - player_transform.translation).normalize_or_zero();
    let attempted_dist = target_pos.distance(player_transform.translation);

    if let Some((_, toi)) = context.cast_ray(
        player_transform.translation,
        ray_direction,
        attempted_dist,
        true,
        QueryFilter {
            groups: Some(Groups::player()),
            ..default()
        },
    ) {
        cam_transform.translation = player_transform.translation - ray_direction + toi * ray_direction; 
    } else {
        cam_transform.translation = player_transform.translation + target_pos;
    }
}
