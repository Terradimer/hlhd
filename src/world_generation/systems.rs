use bevy::{
    prelude::*,
};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::input::components::MainCamera;
use crate::input::resources::{Inputs, MousePosition};
use crate::world_generation::components::{DevInteractable, Dragging, Edges, Resizing};
use crate::world_generation::functions::detect_edge;

use super::{
    COLOR_PLATFORM, SNAP_SCALE, WINDOW_BOTTOM_Y, WINDOW_HEIGHT, WINDOW_LEFT_X, WINDOW_WIDTH,
};

pub fn update_dev_entities(
    mut commands: Commands,
    mouse_position: Res<MousePosition>,
    rapier_context: Res<RapierContext>,
    mut q_interactable: Query<&mut Transform, (With<DevInteractable>, Without<Dragging>)>,
    input: Res<ActionState<Inputs>>,
) {
    if !input.just_pressed(&Inputs::Shoot) {
        return;
    }
    rapier_context.intersections_with_point(
        mouse_position.position,
        QueryFilter::default(),
        |entity| {
            if let Ok(mut transform) = q_interactable.get_mut(entity) {
                let edge = detect_edge(&transform, mouse_position.position);

                if let Some(edge) = edge {
                    commands.entity(entity).insert(Resizing {
                        origin: transform.translation.truncate()
                            + Vec2::new(
                            (transform.scale.x / 2.0) * -(edge.vertical as i32 as f32),
                            (transform.scale.y / 2.0) * -(edge.horizontal as i32 as f32),
                            ),
                        edges: edge,
                    });
                } else {
                    commands.entity(entity).insert(Dragging {
                        offset: mouse_position.position - transform.translation.truncate(),
                    });
                }
            }
            false
        },
    );
}

pub fn dragging_env_entities(
    mut commands: Commands,
    mouse_position: Res<MousePosition>,
    mut q_dragging_entity: Query<Entity, With<Dragging>>,
    mut q_dragging: Query<(&mut Transform, &Dragging)>,
    input: Res<ActionState<Inputs>>,
) {
    if q_dragging_entity.is_empty() {
        return;
    }

    if !input.pressed(&Inputs::Shoot) {
        for entity in q_dragging_entity.iter() {
            commands.entity(entity).remove::<Dragging>();
        }
        return;
    }

    for (mut transform, dragging) in q_dragging.iter_mut() {
        transform.translation =
            (((mouse_position.position - dragging.offset) / SNAP_SCALE).round() * SNAP_SCALE)
                .extend(0.);
    }
}

pub fn scale_dev_entities(
    mut commands: Commands,
    mut q_resizing_entity: Query<Entity, With<Resizing>>,
    mut q_scaling: Query<(&mut Transform, &Resizing)>,
    mouse_position: Res<MousePosition>,
    input: Res<ActionState<Inputs>>,
) {
    if q_scaling.is_empty() {
        return;
    }

    if !input.pressed(&Inputs::Shoot) {
        for entity in q_resizing_entity.iter() {
            commands.entity(entity).remove::<Resizing>();
        }
        return;
    }

    for (mut transform, scaling) in q_scaling.iter_mut() {
        let origin = scaling.origin;
        let diff = ((mouse_position.position - scaling.origin) / SNAP_SCALE).round() * SNAP_SCALE;
        match (scaling.edges.vertical, scaling.edges.horizontal) {
            (_, 0) => {
                transform.scale.x = diff.x.abs();
                transform.translation.x = origin.x + diff.x / 2.;
            }
            (0, _) => {
                transform.scale.y = diff.y.abs();
                transform.translation.y = origin.y + diff.y / 2.;
            }
            _ => {
                transform.scale = diff.abs().extend(0.);
                transform.translation = (origin + diff / 2.).extend(0.);
            }
        };
    }
}

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
) -> (
    SpriteBundle,
    DevInteractable,
    RigidBody,
    Collider,
    CollisionGroups,
    Friction,
) {
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
        DevInteractable,
        RigidBody::Fixed,
        Collider::cuboid(0.5, 0.5),
        crate::collision_groups::Groups::environment(),
        Friction::new(0.),
    )
}
