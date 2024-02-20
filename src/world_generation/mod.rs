use bevy::app::{App, Plugin};
use bevy::prelude::{Color, Startup};

mod components;
mod resources;
mod systems;

pub const WINDOW_WIDTH: f32 = 1024.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;
const COLOR_PLATFORM: Color = Color::rgb(0.75, 0.75, 0.75);

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (systems::make_test_scene));
    }
}
