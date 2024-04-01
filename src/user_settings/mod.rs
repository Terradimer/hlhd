use bevy::app::Plugin;

use self::resources::CameraConfig;

// Replace with settings file
pub const DEFAULT_WINDOW_WIDTH: f32 = 1024.0;
pub const DEFAULT_WINDOW_HEIGHT: f32 = 720.0;
pub struct SettingsPlugin;

pub mod resources;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(CameraConfig { sensitivity: 1. });
    }
}
