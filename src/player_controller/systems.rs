use bevy::{asset::LoadedFolder, prelude::*, render::texture::ImageSampler};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::{action_state::ActionState, InputManagerBundle, prelude::*};

use crate::animation;
use crate::animation::components::{Animation, AnimationIndices};
use crate::input_handler::Inputs;
use crate::macros::{change_state, on_enter, query_guard};
use crate::macros::Init;
use crate::player_controller::JUMP_SRENGTH;

use super::components::*;

pub fn in_air(
    mut commands: Commands,
    mut q_player: Query<
        (Entity, &ContactDirection, &Velocity, Has<Init>), (With<Player>, With<InAir>)
    >,
    mut q_player_sprite: Query<(&PlayerAnimationMap, &mut Animation)>,
    time: Res<Time>,
) {
    let ((p_entity, contacts, vel, init), (index_map, mut animation)) =
        query_guard!(q_player.get_single_mut(), q_player_sprite.get_single_mut());

    on_enter!(commands, p_entity, init, {
        animation.set(&index_map.peak);
    });

    if vel.linvel.y <= -50. {
        animation.set(&index_map.falling);
    }

    if contacts.bottom {
        change_state!(
            commands,
            p_entity,
            InAir,
            Grounded::default(),
            return
        );
    }
}

pub fn grounded(
    mut commands: Commands,
    mut q_player: Query<(Entity, &ContactDirection, &Velocity, &mut Grounded)>,
    mut q_player_sprite: Query<(&PlayerAnimationMap, &mut Animation)>,
    input: Res<ActionState<Inputs>>,
    time: Res<Time>,
) {
    let ((p_entity, contacts, vel, mut state_data), (index_map, mut animation)) =
        query_guard!(q_player.get_single_mut(), q_player_sprite.get_single_mut());

    if vel.linvel.x.abs() > 0. {
        animation.set(&index_map.walk);
    } else {
        animation.set(&index_map.idle);
    }

    if !contacts.bottom {
        if state_data.coyote_time.tick(time.delta()).finished() {
            change_state!(commands, p_entity, Grounded, InAir, {
                animation.set(&index_map.falling);
                return;
            });
        }
    }
    state_data.coyote_time.reset();

    if input.just_pressed(&Inputs::Jump) {
        change_state!(commands, p_entity, Grounded, Jumping, return);
    }
}

pub fn movement_system(
    mut q_player: Query<&mut Velocity, Or<(With<InAir>, With<Grounded>, With<Jumping>)>>,
    mut q_player_sprite: Query<&mut Sprite, With<PlayerAnimationMap>>,
    input: Res<ActionState<Inputs>>,
) {
    let ((mut vel), (mut sprite)) =
        query_guard!(q_player.get_single_mut(), q_player_sprite.get_single_mut());

    let x_axis = input.value(&Inputs::Horizontal);
    if x_axis != 0. {
        sprite.flip_x = x_axis < 0.;
    }
    vel.linvel.x = x_axis * super::PLAYER_SPEED;
}

pub fn jumping(
    mut commands: Commands,
    mut p_query: Query<
        (Entity, &mut Velocity, &mut ExternalImpulse, Has<Init>),
        (With<Player>, With<Jumping>),
    >,
    mut q_player_sprite: Query<(&PlayerAnimationMap, &mut Animation)>,
    input: Res<ActionState<Inputs>>,
) {
    let (
        (p_entity, mut vel, mut applied_impulse, init),
        (index_map, mut animation),
    ) = query_guard!(p_query.get_single_mut(), q_player_sprite.get_single_mut());

    on_enter!(commands, p_entity, init, {
        applied_impulse.impulse.y = JUMP_SRENGTH;
        animation.set(&index_map.rising);
    });

    if !input.pressed(&Inputs::Jump) || vel.linvel.y <= 0. {
        change_state!(commands, p_entity, Jumping, InAir, {
            vel.linvel.y /= 2.;
            return;
        });
    }
}

pub fn contact_detection_system(
    mut q_player: Query<(Entity, &mut ContactDirection), With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    let (p_entity, mut contacts) = query_guard!(q_player.get_single_mut());
    *contacts = ContactDirection::default();
    //rapier_context.contact_pairs_with(p_entity).filter(|pair| pair.raw.collider2.)

    for contact_pair in rapier_context.contact_pairs_with(p_entity) {
        for contact in contact_pair.manifolds() {
            let normal = contact.normal();

            contacts.top |= normal.y < 0.;
            contacts.bottom |= normal.y > 0.;
            contacts.right |= normal.x > 0.;
            contacts.left |= normal.x < 0.;
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    inputs: Res<ActionState<Inputs>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    sprite_handles: Res<animation::resources::PlayerSpriteFolder>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let player_base = commands
        .spawn((
            Player,
            TransformBundle::from_transform(Transform::from_xyz(0., -50., 0.)),
            InputManagerBundle {
                input_map: Inputs::input_map(),
                ..default()
            },
            InAir,
            InheritedVisibility::default(),
        ))
        .id();

    let sprite_handler = commands
        .spawn(load_player_sprites(
            asset_server,
            texture_atlas_layouts,
            sprite_handles,
            loaded_folders,
            textures,
        ))
        .id();

    let physics = (
        RigidBody::Dynamic,
        Velocity::zero(),
        ExternalForce {
            force: Vect::splat(0.),
            torque: 0.,
        },
        ExternalImpulse {
            impulse: Vect::splat(0.),
            torque_impulse: 0.,
        },
        ContactDirection::default(),
        Ccd { enabled: true },
        Friction::new(0.),
        Collider::cuboid(35. / 2., 60. / 2.),
        CollisionGroups {
            memberships: crate::collision_groups::Groups::PLAYER,
            filters: crate::collision_groups::Groups::ENVIRONMENT,
        },
        GravityScale(9.),
        LockedAxes::ROTATION_LOCKED_Z,
    );

    commands
        .entity(player_base)
        .add_child(sprite_handler)
        .insert(physics);
}

fn load_player_sprites(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    sprite_handles: Res<animation::resources::PlayerSpriteFolder>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut textures: ResMut<Assets<Image>>,
) -> (SpriteSheetBundle, Animation, PlayerAnimationMap) {
    let loaded_folder = loaded_folders.get(&sprite_handles.handle).unwrap();

    let (layout, linear_texture) = animation::systems::create_texture_atlas(
        loaded_folder,
        Some(UVec2 { x: 0, y: 5 }),
        Some(ImageSampler::nearest()),
        &mut textures,
    );
    let animations = PlayerAnimationMap {
        idle: animation::macros::add_animation!(
            "textures/demo_player/idle/1.png",
            6,
            asset_server,
            layout
        ),
        falling: animation::macros::add_animation!(
            "textures/demo_player/in_air/3.png",
            0,
            asset_server,
            layout
        ),
        rising: animation::macros::add_animation!(
            "textures/demo_player/in_air/1.png",
            0,
            asset_server,
            layout
        ),
        peak: animation::macros::add_animation!(
            "textures/demo_player/in_air/2.png",
            0,
            asset_server,
            layout
        ),
        walk: animation::macros::add_animation!(
            "textures/demo_player/walk/1.png",
            7,
            asset_server,
            layout
        ),
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
