use super::{
    components::*,
    events::{LoadRoomEvent, SaveRoomEvent},
    functions::*,
    MIN_SCALE, SNAP_SCALE, WINDOW_BOTTOM_Y, WINDOW_HEIGHT, WINDOW_LEFT_X, WINDOW_WIDTH,
};
use crate::input::resources::{Inputs, MousePosition};
use bevy::{
    math::vec3,
    prelude::*,
    scene::ron::{
        self, from_str,
        ser::{to_string_pretty, PrettyConfig},
    },
    tasks::IoTaskPool,
    utils::warn,
};
use bevy_ecs::{query, world};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use rfd::FileDialog;
use std::{fs::File, io::Write};

pub fn update_dev_entities(
    mut commands: Commands,
    mouse_position: Res<MousePosition>,
    rapier_context: Res<RapierContext>,
    mut q_interactable: Query<
        (&Transform, Has<Draggable>, Has<Scalable>),
        Or<(With<Draggable>, With<Scalable>)>,
    >,
    input: Res<ActionState<Inputs>>,
) {
    if !input.just_pressed(&Inputs::Shoot) {
        return;
    }
    rapier_context.intersections_with_point(
        mouse_position.position,
        QueryFilter::default(),
        |entity| {
            if let Ok((transform, draggable, scalable)) = q_interactable.get_mut(entity) {
                if scalable {
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
                        return false;
                    }
                }
                if draggable {
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
    q_dragging_entity: Query<Entity, With<Dragging>>,
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
        let z = transform.translation.z;
        transform.translation =
            (((mouse_position.position - dragging.offset) / SNAP_SCALE).round() * SNAP_SCALE)
                .extend(z);
    }
}

pub fn scale_dev_entities(
    mut commands: Commands,
    q_resizing_entity: Query<Entity, With<Resizing>>,
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

        let new_scale = diff.abs().extend(0.);

        match (scaling.edges.vertical, scaling.edges.horizontal) {
            (_, 0) => {
                if new_scale.x > MIN_SCALE {
                    transform.scale.x = new_scale.x;
                    transform.translation.x = origin.x + diff.x / 2.;
                }
            }
            (0, _) => {
                if new_scale.y > MIN_SCALE {
                    transform.scale.y = new_scale.y;
                    transform.translation.y = origin.y + diff.y / 2.;
                }
            }
            _ => {
                if new_scale.x > MIN_SCALE && new_scale.y > MIN_SCALE {
                    // Check both components
                    transform.scale = new_scale;
                    let z = transform.translation.z;
                    transform.translation = (origin + diff / 2.).extend(z);
                }
            }
        };
    }
}

pub fn make_test_scene(mut ev_loadcall: EventWriter<LoadRoomEvent>) {
    let load_path = FileDialog::new()
        .add_filter("RON file", &["ron"])
        .pick_file();

    ev_loadcall.send(LoadRoomEvent { path: load_path });
}

pub fn save_room(
    mut ev_savecall: EventReader<SaveRoomEvent>,
    q_saveable: Query<(Entity, &Transform), With<Saveable>>,
) {
    if ev_savecall.is_empty() {
        return;
    }

    let save_path = FileDialog::new()
        .add_filter("RON file", &["ron"])
        .save_file();

    if let Some(path) = save_path {
        let mut entity_data: Vec<EntityData> = Vec::new();

        for (_, transform) in q_saveable.iter() {
            entity_data.push(EntityData {
                position: transform.translation,
                scale: transform.scale,
            });
        }

        let mut data = std::collections::HashMap::new();
        data.insert("platforms", entity_data);

        let pretty_config = PrettyConfig::new();
        let ron_string = to_string_pretty(&data, pretty_config).expect("Failed to serialize data");

        std::fs::write(path, ron_string).expect("Failed to write to file");
    }

    ev_savecall.clear()
}

pub fn load_room(mut commands: Commands, mut ev_loadcall: EventReader<LoadRoomEvent>) {
    if ev_loadcall.is_empty() {
        return;
    }

    let load_path = &ev_loadcall.read().next().unwrap().path;

    if let Some(path) = load_path {
        let file_contents = std::fs::read_to_string(path).expect("Failed to read file");

        let data: std::collections::HashMap<String, Vec<EntityData>> =
            from_str(&file_contents).expect("Failed to deserialize RON data");

        if let Some(platforms) = data.get("platforms") {
            for platform in platforms {
                commands.spawn(gen_platform(platform.position, platform.scale));
            }
        } else {
            println!("No 'platforms' data found in the file");
        }
    }

    ev_loadcall.clear()
}