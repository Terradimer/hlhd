use bevy::{asset::LoadedFolder, prelude::*};
use bevy_ecs::prelude::Res;

use crate::animation::components::Animation;
use crate::animation::resources::*;

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut Animation,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut animation, mut sprite) in &mut query {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            sprite.index = if animation.indicies.first <= sprite.index && sprite.index < animation.indicies.last {
                sprite.index + 1
            } else {
                animation.indicies.first
            };
        }
    }
}

pub fn check_textures(
    mut next_state: ResMut<NextState<super::AppState>>,
    sprite_folder: ResMut<PlayerSpriteFolder>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(&sprite_folder.handle) {
            next_state.set(super::AppState::Loaded);
        }
    }
}

pub fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PlayerSpriteFolder {
        handle: asset_server.load_folder("textures/demo_player")
    });
}