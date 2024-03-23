use std::path::PathBuf;

use bevy_ecs::event::Event;

#[derive(Event)]
pub struct SaveRoomEvent;

#[derive(Event)]
pub struct LoadRoomEvent {
    pub path: Option<PathBuf>,
}
