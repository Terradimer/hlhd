use bevy::app::{App, Update};
use bevy::prelude::Plugin;
use bevy_ecs::prelude::{in_state, IntoSystemConfigs, OnEnter};

use crate::AppState;

pub mod systems;
pub mod resources;
pub mod components;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Loading), systems::load_textures)
            .add_systems(Update, systems::check_textures.run_if(in_state(AppState::Loading)))
            .add_systems(Update, systems::animate_sprite);
    }
}