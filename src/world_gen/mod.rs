use bevy::app::{App, Plugin, Startup, Update};

mod resources;
mod rooms;
mod systems;

use systems::*;

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_scene);
    }
}
