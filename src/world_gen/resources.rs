use std::path::PathBuf;

use bevy_ecs::system::Resource;

#[derive(Resource)]
pub struct LoadRequest {
    pub path: Option<PathBuf>,
}
