use bevy::{
    asset::LoadedFolder,
    prelude::{Handle, Resource},
};

#[derive(Resource, Default)]
pub struct PlayerSpriteFolder {
    pub handle: Handle<LoadedFolder>,
}