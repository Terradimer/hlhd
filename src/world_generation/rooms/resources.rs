use super::components::Room;
use bevy_ecs::system::Resource;

#[derive(Resource)]
pub struct CurrentRoom {
    pub room: Room,
}
