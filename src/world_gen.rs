use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct MainCamera;

pub const WINDOW_WIDTH: f32 = 1024.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;
const COLOR_PLATFORM: Color = Color::rgb(0.75, 0.75, 0.75);

pub fn make_test_scene(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    commands.spawn(gen_platform(
        Vec3::new(0., WINDOW_BOTTOM_Y, 0.),
        Vec3::new(WINDOW_WIDTH, 50., 1.),
    ));
    commands.spawn(gen_platform(
        Vec3::new(WINDOW_LEFT_X, 0., 0.),
        Vec3::new(25., WINDOW_HEIGHT, 1.),
    ));
    commands.spawn(gen_platform(
        Vec3::new(-WINDOW_LEFT_X, 0., 0.),
        Vec3::new(25., WINDOW_HEIGHT, 1.),
    ));
    commands.spawn(gen_platform(
        Vec3::new(-250., WINDOW_BOTTOM_Y + 150., 0.),
        Vec3::new(300., 30., 1.),
    ));
    commands.spawn(gen_platform(
        Vec3::new(250., WINDOW_BOTTOM_Y + 150., 0.),
        Vec3::new(300., 30., 1.),
    ));
    commands.spawn(gen_platform(
        Vec3::new(0., 40., 0.),
        Vec3::new(250., 30., 1.),
    ));
}

pub fn gen_platform(
    _translation: Vec3,
    _scale: Vec3,
) -> (SpriteBundle, RigidBody, Collider, CollisionGroups, Friction) {
    (
        SpriteBundle {
            sprite: Sprite {
                color: COLOR_PLATFORM,
                ..Default::default()
            },
            transform: Transform {
                translation: _translation,
                scale: _scale,
                ..Default::default()
            },
            ..Default::default()
        },
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5),
        crate::collision_groups::GroupsConfig::ground_group(),
        Friction::new(0.),
    )
}