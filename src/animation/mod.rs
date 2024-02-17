use bevy::app::{App, Startup, Update};
use bevy::prelude::Plugin;

pub mod systems;
pub mod resources;
pub mod components;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, systems::demo_setup)
            .add_systems(Update, systems::animate_sprite);
    }
}