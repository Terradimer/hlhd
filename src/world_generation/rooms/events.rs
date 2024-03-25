use std::path::PathBuf;

use bevy_ecs::event::Event;

#[derive(Event)]
pub struct SaveRoomEvent;

#[derive(Event, Clone)]
pub struct LoadRoomEvent {
    pub path: Option<PathBuf>,
}
