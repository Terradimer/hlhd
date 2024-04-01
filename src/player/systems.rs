use bevy::{asset::LoadedFolder, prelude::*};
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::{action_state::ActionState, prelude::*, InputManagerBundle};

use super::{JUMP_STRENGTH, PLAYER_SPEED};
use crate::camera::components::{CameraAnchor, MainCamera};
use crate::collision_groups::Groups;
use crate::input::Inputs;
use crate::macros::{on_enter, query_guard};
use crate::state_machines::*;
use crate::time::resources::ScaledTime;

use super::components::*;

pub fn in_air(
    mut commands: Commands,
    mut q_player: Query<
        (Entity, &ContactDirection, &Velocity, Has<JustEntered>),
        (With<Player>, With<InAir>),
    >,
) {
    let (p_entity, contacts, vel, init) = query_guard!(q_player.get_single_mut());

    if contacts.bottom {
        change_state::<InAir, Grounded>(&mut commands, p_entity, Grounded::default());
    }
}

pub fn grounded(
    mut commands: Commands,
    mut q_player: Query<(Entity, &ContactDirection, &Velocity, &mut Grounded)>,
    input: Res<ActionState<Inputs>>,
    time: Res<Time>,
) {
    let (p_entity, contacts, vel, mut state_data) = query_guard!(q_player.get_single_mut());

    if !contacts.bottom {
        if state_data.coyote_time.tick(time.delta()).finished() {
            change_state::<Grounded, InAir>(&mut commands, p_entity, InAir);
            return;
        }
    } else {
        state_data.coyote_time.reset();
    }

    if input.just_pressed(&Inputs::Jump) {
        change_state::<Grounded, Jumping>(&mut commands, p_entity, Jumping);
    }
}

pub fn movement_system(
    mut q_player: Query<&mut Velocity, Or<(With<InAir>, With<Grounded>, With<Jumping>)>>,
    q_cam: Query<&Transform, With<MainCamera>>,
    input: Res<ActionState<Inputs>>,
    time: Res<ScaledTime>,
) {
    let (mut vel, camera_transform) = query_guard!(q_player.get_single_mut(), q_cam.get_single());

    let movement_axis = input
        .clamped_axis_pair(&Inputs::Movement)
        .unwrap()
        .xy()
        .normalize_or_zero();
    let yaw = camera_transform.rotation.to_euler(EulerRot::YXZ).0;
    let direction =
        Quat::from_rotation_y(yaw).mul_vec3(Vec3::new(movement_axis.x, 0., -movement_axis.y));

    vel.linvel.x = direction.x * PLAYER_SPEED * time.delta.as_secs_f32();
    vel.linvel.z = direction.z * PLAYER_SPEED * time.delta.as_secs_f32();
}

pub fn jumping(
    mut commands: Commands,
    mut p_query: Query<
        (
            Entity,
            &mut Velocity,
            &mut ExternalImpulse,
            Has<JustEntered>,
        ),
        (With<Player>, With<Jumping>),
    >,
    input: Res<ActionState<Inputs>>,
) {
    let (p_entity, mut vel, mut applied_impulse, init) = query_guard!(p_query.get_single_mut());

    on_enter!(commands, p_entity, init, {
        applied_impulse.impulse.y = JUMP_STRENGTH;
        return;
    });

    if !input.pressed(&Inputs::Jump) || vel.linvel.y <= 0. {
        change_state::<Jumping, InAir>(&mut commands, p_entity, InAir);
        vel.linvel.y /= 2.;
        applied_impulse.impulse.y /= 2.;
        return;
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

            contacts.top |= normal.y < 0.;
            contacts.bottom |= normal.y > 0.;
            contacts.right |= normal.x > 0.;
            contacts.left |= normal.x < 0.;
            contacts.front |= normal.z > 0.;
            contacts.back |= normal.z < 0.;
        }
    }

    //println!("{contacts:?}")
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_height: f32 = 2.5;
    let start_pos = Vec3::new(0., 10., 0.);

    let player_base = commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_translation(start_pos)),
            Player,
            InAir,
        ))
        .id();

    let model = commands
        .spawn((PbrBundle {
            mesh: meshes.add(Capsule3d::new(player_height / 2., player_height)),
            transform: Transform::default(),
            material: materials.add(Color::WHITE),
            ..default()
        },))
        .id();

    let physics = (
        RigidBody::Dynamic,
        Velocity::zero(),
        ExternalForce {
            force: Vect::splat(0.),
            torque: Vec3::ZERO,
        },
        ExternalImpulse {
            impulse: Vect::splat(0.),
            torque_impulse: Vec3::ZERO,
        },
        ContactDirection::default(),
        Ccd { enabled: true },
        Friction::new(0.),
        Collider::capsule_y(player_height / 2., player_height / 4.),
        Groups::player(),
        GravityScale(9.),
        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
    );

    commands
        .entity(player_base)
        .add_child(model)
        .insert(physics);
}
