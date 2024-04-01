use bevy::app::Plugin;

// Replace with settings file
pub const DEFAULT_WINDOW_WIDTH: f32 = 1024.0;
pub const DEFAULT_WINDOW_HEIGHT: f32 = 720.0;
pub const MOUSE_SENSITIVITY:f32 = 1.;
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {}
}
