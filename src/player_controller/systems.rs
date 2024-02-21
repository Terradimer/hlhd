use bevy::{asset::LoadedFolder, prelude::*, render::texture::ImageSampler};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::{action_state::ActionState, InputManagerBundle, prelude::*};

use crate::animation;
use crate::animation::components::{Animation, AnimationIndices};
use crate::input_handler::Inputs;
use crate::macros::query_guard;

use super::components::*;

pub fn in_air(
    mut commands: Commands,
    mut q_player: Query<(Entity, &ContactDirection, &Velocity, &InAirState), With<Player>>,
    mut q_player_sprite: Query<(&PlayerIndexMap, &mut Animation)>,
    input: Res<ActionState<Inputs>>,
    time: Res<Time>,
) {
    let (p_entity, contacts, vel, state) = query_guard!(q_player.get_single_mut());

    if contacts.bottom {
        commands
            .entity(p_entity)
            .remove::<InAirState>()
            .insert(GroundedState);
    }
}

pub fn grounded(
    mut commands: Commands,
    mut q_player: Query<(Entity, &ContactDirection, &Velocity), With<GroundedState>>,
    mut q_player_sprite: Query<(&PlayerIndexMap, &mut Animation)>,
) {
    let (p_entity, contacts, vel) = query_guard!(q_player.get_single_mut());
    let (index_map, mut animation) = query_guard!(q_player_sprite.get_single_mut());

    if vel.linvel.x.abs() > 0. {
        animation.indicies = index_map.walk.indicies;
    } else {
        animation.indicies = index_map.idle.indicies;
    }

    if !contacts.bottom {
        commands
            .entity(p_entity)
            .remove::<GroundedState>()
            .insert(InAirState { coyote_time: 0.2 });
    }
}

pub fn contact_detection_system(
    mut q_player: Query<(Entity, &mut ContactDirection), With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    let (p_entity, mut contacts) = query_guard!(q_player.get_single_mut());
    *contacts = ContactDirection::default();

    for contact_pair in rapier_context.contact_pairs_with(p_entity) {
        for contact in contact_pair.manifolds() {
            let normal = contact.normal();

            contacts.top |= normal.y < 0.;
            contacts.bottom |= normal.y > 0.;
            contacts.right |= normal.x > 0.;
            contacts.left |= normal.x < 0.;
        }
    }
    // println!("{:?}", contacts); // Debug
}

pub fn movement_system(
    mut q_player: Query<&mut Velocity, Or<(With<InAirState>, With<GroundedState>)>>,
    mut q_player_sprite: Query<&mut Sprite, With<PlayerIndexMap>>,
    input: Res<ActionState<Inputs>>,
) {
    let (Ok(mut vel), Ok(mut sprite)) =
        (q_player.get_single_mut(), q_player_sprite.get_single_mut())
        else {
            return;
        };

    let x_axis = input.value(&Inputs::Horizontal);
    if x_axis != 0. {
        sprite.flip_x = x_axis < 0.;
    }
    vel.linvel.x = x_axis * super::PLAYER_SPEED;
}

fn jumping(mut query: Query<(), With<Player>>, time: Res<Time>) {}

pub fn spawn_player(
    mut commands: Commands,
    inputs: Res<ActionState<Inputs>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    sprite_handles: Res<animation::resources::PlayerSpriteFolder>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let p_sprite_handler = commands
        .spawn(load_player_sprites(
            asset_server,
            texture_atlas_layouts,
            sprite_handles,
            loaded_folders,
            textures,
        ))
        .id();

    commands
        .spawn((
            Player,
            Sprite::default(),
            TransformBundle::from_transform(Transform::from_xyz(0., -50., 0.)),
            InputManagerBundle {
                input_map: Inputs::input_map(),
                ..default()
            },
            GravityScale(9.),
            LockedAxes::ROTATION_LOCKED_Z,
            RigidBody::Dynamic,
            Velocity::zero(),
            ContactDirection::default(),
            Ccd { enabled: true },
            InAirState { coyote_time: 0. },
            Friction::new(0.),
            Collider::cuboid(35. / 2., 60. / 2.),
            InheritedVisibility::default(),
        ))
        .add_child(p_sprite_handler);
}

fn load_player_sprites(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    sprite_handles: Res<animation::resources::PlayerSpriteFolder>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut textures: ResMut<Assets<Image>>,
) -> (SpriteSheetBundle, Animation, PlayerIndexMap) {
    let loaded_folder = loaded_folders.get(&sprite_handles.handle).unwrap();

    let (layout, linear_texture) = animation::systems::create_texture_atlas(
        loaded_folder,
        None,
        Some(ImageSampler::linear()),
        &mut textures,
    );
    let animations = PlayerIndexMap {
        idle: animation::macros::add_animation!("textures/demo_player/idle/1.png", 6, asset_server, layout),
        falling: animation::macros::add_animation!("textures/demo_player/in_air/3.png", 0, asset_server, layout),
        walk: animation::macros::add_animation!("textures/demo_player/walk/1.png", 7, asset_server, layout),
    };
    return (
        animation::systems::create_sprite_from_atlas(
            (0., 45., 0.),
            1.75,
            animations.falling.indicies.first,
            texture_atlas_layouts.add(layout.clone()),
            linear_texture,
        ),
        animations.falling.clone(),
        animations,
    );
}
