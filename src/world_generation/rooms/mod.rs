use bevy::app::{Plugin, Update};
use bevy_ecs::schedule::{common_conditions::in_state, IntoSystemConfigs, OnExit};
use systems::*;

use crate::AppState;

use self::{
    components::{Room, RoomBounds},
    events::*,
    resources::CurrentRoom,
};

pub mod components;
pub mod events;
pub mod functions;
pub mod resources;
pub mod systems;

pub struct RoomsPlugin;

impl Plugin for RoomsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(CurrentRoom {
            room: Room {
                state: components::RoomState::Cached,
                bounds: RoomBounds::default(),
                neighbors: Vec::new(),
            },
        })
        .add_systems(
            Update,
            (save_room, load_room).run_if(in_state(AppState::Dev)),
        )
        .add_systems(OnExit(AppState::Dev), update_room_bounds)
        .add_event::<SaveRoomEvent>()
        .add_event::<LoadRoomEvent>();
    }
}
