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
