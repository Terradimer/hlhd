use bevy::asset::LoadedFolder;
use bevy::prelude::*;

use crate::animation::components::Animation;
use crate::animation::resources::PlayerSpriteFolder;
use crate::time::resources::ScaledTime;

pub fn animate_sprites(
    time: Res<ScaledTime>,
    mut query: Query<(&mut Animation, &mut TextureAtlas)>,
) {
    for (mut animation, mut sprite) in &mut query {
        animation.timer.tick(time.delta);
        if animation.timer.just_finished() {
            sprite.index = if animation.indices.first <= sprite.index
                && sprite.index < animation.indices.last
            {
                sprite.index + 1
            } else {
                animation.indices.first
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
            next_state.set(super::AppState::Playing);
        }
    }
}

pub fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PlayerSpriteFolder {
        handle: asset_server.load_folder("textures/demo_player"),
    });
}
