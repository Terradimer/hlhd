use bevy::{math::Vec3, reflect::Reflect};
use bevy_ecs::component::Component;
use serde::{Deserialize, Serialize};

#[derive(Reflect, Serialize, Deserialize)]
pub enum SaveData {
    Platform { position: Vec3, scale: Vec3 },
    Exit { position: Vec3, scale: Vec3 },
    Player { position: Vec3 },
}

#[derive(Component, Reflect, Serialize, Deserialize)]
pub enum Saveable {
    Platform,
    Exit,
    Player,
}

pub enum RoomState {
    Cached,
    Loaded,
}

pub struct RoomBounds {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

impl Default for RoomBounds {
    fn default() -> Self {
        Self {
            min_x: f32::MAX,
            max_x: f32::MIN,
            min_y: f32::MAX,
            max_y: f32::MIN,
        }
    }
}

#[derive(Component)]
pub struct Room {
    pub state: RoomState,
    pub bounds: RoomBounds,
    pub neighbors: Vec<Room>,
}

#[derive(Component)]
pub struct RoomBoundsAffector;
