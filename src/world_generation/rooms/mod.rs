use bevy::app::{Plugin, Update};
use systems::*;

use self::events::*;

pub mod components;
pub mod events;
pub mod systems;

pub struct RoomsPlugin;

impl Plugin for RoomsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (save_room, load_room))
            .add_event::<SaveRoomEvent>()
            .add_event::<LoadRoomEvent>();
    }
}
