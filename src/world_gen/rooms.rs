use std::path::PathBuf;

use bevy::{prelude::*, scene::ron::from_str};
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Room {
    pub state: RoomState,
    pub structure: Vec<RoomObjectData>,
}

pub enum RoomState {
    Loaded,
    Unloaded,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomObjectData {
    transform: Transform,
    // Need to introduce model asset serialization
    // model_type: ModelType,
}

fn deserialize_room(path: PathBuf) -> Room {
    let file_contents = std::fs::read_to_string(path).expect("Failed to read file");

    Room {
        state: RoomState::Unloaded,
        structure: from_str(&file_contents).expect("Failed to deserialize RON data"),
    }
}
