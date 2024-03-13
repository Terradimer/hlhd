use crate::animation::{components::Animation, functions::*, resources::PlayerSpriteFolder};
use crate::player_controller::components::PlayerAnimationMap;
use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy::render::texture::ImageSampler;

pub(crate) fn load_player_sprites(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    sprite_handles: Res<PlayerSpriteFolder>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut textures: ResMut<Assets<Image>>,
) -> (SpriteSheetBundle, Animation, PlayerAnimationMap) {
    let loaded_folder = loaded_folders.get(&sprite_handles.handle).unwrap();

    let (layout, linear_texture) = create_texture_atlas(
        loaded_folder,
        Some(UVec2 { x: 0, y: 5 }),
        Some(ImageSampler::nearest()),
        &mut textures,
    );
    let animations = PlayerAnimationMap {
        idle: add_animation(
            asset_server
                .get_handle("textures/demo_player/idle/1.png")
                .unwrap(),
            6,
            &layout,
        ),
        falling: add_animation(
            asset_server
                .get_handle("textures/demo_player/in_air/3.png")
                .unwrap(),
            0,
            &layout,
        ),
        rising: add_animation(
            asset_server
                .get_handle("textures/demo_player/in_air/1.png")
                .unwrap(),
            0,
            &layout,
        ),
        peak: add_animation(
            asset_server
                .get_handle("textures/demo_player/in_air/2.png")
                .unwrap(),
            0,
            &layout,
        ),
        walk: add_animation(
            asset_server
                .get_handle("textures/demo_player/walk/1.png")
                .unwrap(),
            7,
            &layout,
        ),
    };
    (
        create_sprite_from_atlas(
            (0., 45., 0.),
            1.75,
            animations.falling.indices.first,
            texture_atlas_layouts.add(layout),
            linear_texture,
        ),
        animations.falling.clone(),
        animations,
    )
}
