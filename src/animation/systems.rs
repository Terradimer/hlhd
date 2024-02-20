use bevy::{asset::LoadedFolder, prelude::*};
use bevy::render::texture::ImageSampler;
use bevy_ecs::prelude::Res;

use crate::animation::components::Animation;
use crate::animation::resources::*;

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut Animation,
        &mut TextureAtlas,
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

pub fn create_sprite_from_atlas(
    translation: (f32, f32, f32),
    scale: f32,
    sprite_index: usize,
    atlas_handle: Handle<TextureAtlasLayout>,
    texture: Handle<Image>,
) -> SpriteSheetBundle {
    SpriteSheetBundle {
        transform: Transform {
            translation: Vec3::new(translation.0, translation.1, translation.2),
            scale: Vec3::splat(scale),
            ..default()
        },
        texture,
        atlas: TextureAtlas {
            index: sprite_index,
            layout: atlas_handle,
        },
        ..default()
    }
}

pub fn create_texture_atlas(
    folder: &LoadedFolder,
    padding: Option<UVec2>,
    sampling: Option<ImageSampler>,
    textures: &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, Handle<Image>) {
    // Build a texture atlas using the individual sprites
    let mut texture_atlas_builder =
        TextureAtlasBuilder::default().padding(padding.unwrap_or_default());
    for handle in folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
        let Some(texture) = textures.get(id) else {
            warn!(
                "{:?} did not resolve to an `Image` asset.",
                handle.path().unwrap()
            );
            continue;
        };

        texture_atlas_builder.add_texture(Some(id), texture);
    }

    let (texture_atlas_layout, texture) = texture_atlas_builder.finish().unwrap();
    let texture = textures.add(texture);

    // Update the sampling settings of the texture atlas
    let image = textures.get_mut(&texture).unwrap();
    image.sampler = sampling.unwrap_or_default();

    (texture_atlas_layout, texture)
}
