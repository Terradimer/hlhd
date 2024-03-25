use bevy::{asset::LoadedFolder, prelude::*};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::{action_state::ActionState, prelude::*, InputManagerBundle};

use crate::animation;
use crate::animation::components::Animation;
use crate::input::resources::Inputs;
use crate::macros::{change_state, on_enter, query_guard, Init};
use crate::player_controller::functions::load_player_sprites;
use crate::player_controller::JUMP_STRENGTH;
use crate::world_generation::components::Draggable;

use super::components::*;

pub fn in_air(
    mut commands: Commands,
    mut q_player: Query<
        (Entity, &ContactDirection, &Velocity, Has<Init>),
        (With<Player>, With<InAir>),
    >,
    mut q_player_sprite: Query<(&PlayerAnimationMap, &mut Animation)>,
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
        change_state!(commands, p_entity, InAir, Grounded::default(), return);
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
    } else {
        state_data.coyote_time.reset();
    }

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
    let ((p_entity, mut vel, mut applied_impulse, init), (index_map, mut animation)) =
        query_guard!(p_query.get_single_mut(), q_player_sprite.get_single_mut());

    on_enter!(commands, p_entity, init, {
        applied_impulse.impulse.y = JUMP_STRENGTH;
        animation.set(&index_map.rising);
        return;
    });

    if !input.pressed(&Inputs::Jump) || vel.linvel.y <= 0. {
        change_state!(commands, p_entity, Jumping, InAir, {
            vel.linvel.y /= 2.;
            applied_impulse.impulse.y /= 2.;
            return;
        });
    }
}

pub fn contact_detection(
    mut q_player: Query<(Entity, &mut ContactDirection), With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    let (p_entity, mut contacts) = query_guard!(q_player.get_single_mut());
    *contacts = ContactDirection::default();

    for contact_pair in rapier_context.contact_pairs_with(p_entity) {
        for contact in contact_pair.manifolds() {
            let normal = contact.normal();

            contacts.top |= normal.y > 0.;
            contacts.bottom |= normal.y < 0.;
            contacts.right |= normal.x > 0.;
            contacts.left |= normal.x < 0.;
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    sprite_handles: Res<animation::resources::PlayerSpriteFolder>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    textures: ResMut<Assets<Image>>,
) {
    let player_base = commands
        .spawn((
            Player,
            Draggable,
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
