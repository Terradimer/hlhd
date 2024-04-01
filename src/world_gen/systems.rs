use bevy::{prelude::*, scene::ron::from_str};
use bevy_rapier3d::geometry::Collider;
use rfd::FileDialog;

use crate::collision_groups::Groups;

use super::{
    resources::LoadRequest,
    rooms::{Room, RoomState},
};

pub fn start_scene(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>, 
    // mut load_request: ResMut<LoadRequest>
) {
    // load_request.path = FileDialog::new()
    //     .add_filter("RON file", &["ron"])
    //     .pick_file();
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(100.0, 1.0, 100.0)),
            material: materials.add(Color::rgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 0., 0.0),
            ..default()
        },
        Groups::ENVIRONMENT,
        Collider::cuboid(50., 0.5, 50.)
    ));

    // commands.spawn(
    //     PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 20000000.,
    //         radius: 100.,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(0.0, 10.0, 0.0),
    //     ..default()
    // });
}

// pub fn load_room(mut commands: Commands) {}
