use bevy::math::Vec3;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Draggable;

#[derive(Component)]
pub struct Scalable;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub(crate) struct Dragging {
    pub(crate) offset: Vec2,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Resizing {
    pub(crate) edges: Edges,
    pub(crate) origin: Vec2,
}

pub struct Edges {
    pub(crate) horizontal: i8,
    pub(crate) vertical: i8,
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct EntityData {
    pub(crate) position: Vec3,
    pub(crate) scale: Vec3,
}

#[derive(Component, Reflect, Default, Serialize, Deserialize)]
pub struct Saveable;
