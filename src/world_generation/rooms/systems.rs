use bevy::{
    hierarchy::BuildChildren,
    render::view::InheritedVisibility,
    scene::ron::{
        from_str,
        ser::{to_string_pretty, PrettyConfig},
    },
    transform::{components::Transform, TransformBundle},
};
use bevy_ecs::{
    event::EventReader,
    system::{Commands, Query},
};
use rfd::FileDialog;

use crate::world_generation::functions::gen_platform;

use super::{
    components::{SaveData, Saveable},
    events::SaveRoomEvent,
    LoadRoomEvent,
};

pub fn save_room(
    mut ev_savecall: EventReader<SaveRoomEvent>,
    q_saveable: Query<(&Transform, &Saveable)>,
) {
    if ev_savecall.is_empty() {
        return;
    }

    let save_path = FileDialog::new()
        .add_filter("RON file", &["ron"])
        .save_file();

    let mut data: Vec<SaveData> = Vec::new();

    let Some(path) = save_path else {
        return;
    };

    for (transform, save_type) in q_saveable.iter() {
        match save_type {
            Saveable::Platform => data.push(SaveData::Platform {
                position: transform.translation,
                scale: transform.scale,
            }),
            Saveable::Exit => data.push(SaveData::Exit {
                position: transform.translation,
                scale: transform.scale,
            }),
            Saveable::Player => {}
        }
    }

    let pretty_config = PrettyConfig::new();
    let ron_string = to_string_pretty(&data, pretty_config).expect("Failed to serialize data");

    std::fs::write(path, ron_string).expect("Failed to write to file");

    ev_savecall.clear()
}

pub fn load_room(mut commands: Commands, mut ev_loadcall: EventReader<LoadRoomEvent>) {
    if ev_loadcall.is_empty() {
        return;
    }

    // this is some stupid shit but its only called once at a time so whatever
    let load_path = ev_loadcall.read().cloned().last().unwrap().path;
    ev_loadcall.clear();

    if let Some(path) = load_path {
        let file_contents = std::fs::read_to_string(path).expect("Failed to read file");

        let data: Vec<SaveData> = from_str(&file_contents).expect("Failed to deserialize RON data");

        commands
            .spawn((TransformBundle::default(), InheritedVisibility::VISIBLE))
            .with_children(|parent| {
                for loaded_data in data {
                    match loaded_data {
                        SaveData::Platform { position, scale } => {
                            parent.spawn(gen_platform(position, scale));
                        }
                        SaveData::Exit { position, scale } => {}
                        SaveData::Player { position } => {}
                    }
                }
            });
    }
}
