use bevy::prelude::{Component, Vec2};

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

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Focused;