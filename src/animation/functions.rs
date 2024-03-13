use crate::animation::components::{Animation, AnimationIndices};
use bevy::asset::{Assets, Handle, LoadedFolder};
use bevy::log::warn;
use bevy::math::UVec2;
use bevy::prelude::*;
use bevy::render::texture::ImageSampler;
use bevy_ecs::change_detection::ResMut;

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

pub fn add_animation(
    image: Handle<Image>,
    num_frames: usize,
    texture_atlas: &TextureAtlasLayout,
) -> Animation {
    let index = texture_atlas.get_texture_index(image).unwrap();

    Animation {
        timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        indices: AnimationIndices {
            first: index,
            last: index + num_frames,
        },
    }
}
