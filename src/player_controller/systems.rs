use bevy::a11y::accesskit::Role::Timer;
use crate::input_handler::Inputs;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::{action_state::ActionState, prelude::*, InputManagerBundle};
use crate::animation::components::AnimationTimer;

use super::components::{InAirData, Player};
use super::resources::{PlayerState, PreviousState};

pub fn enter_in_air(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    prev_state: Res<PreviousState>,
) {
    let player_entity = player_query.single();
    let coyote_time = match prev_state.state {
        Some(PlayerState::Grounded) => 0.2,
        _ => 0.,
    };

    commands
        .entity(player_entity)
        .insert(InAirData { coyote_time });
}

pub fn exit_in_air(
    mut commands: Commands,
    mut prev_state: ResMut<PreviousState>,
    state: Res<State<PlayerState>>,
    player_query: Query<Entity, With<Player>>,
) {
    prev_state.state = Some(*state.get());
    let player_entity = player_query.single();
    commands.entity(player_entity).remove::<InAirData>();
}

pub fn update_in_air(
    mut query: Query<(&mut Velocity,), With<Player>>,
    input: Res<ActionState<Inputs>>,
    time: Res<Time>,
) {
}

pub fn update_grounded(
    mut query: Query<(&mut Velocity,), With<Player>>,
    input: Res<ActionState<Inputs>>,
    time: Res<Time>,
) {
}

pub fn movement_system(
    mut query: Query<(&mut Velocity), With<Player>>,
    input: Res<ActionState<Inputs>>,
) {
    let Ok((mut vel)) = query.get_single_mut() else {
        return;
    };

    let x_axis = input.value(Inputs::Horizontal);
    vel.linvel.x = x_axis * super::PLAYER_SPEED;
}

fn update_jumping(
    mut query: Query<(), With<Player>>,
    time: Res<Time>,
    mut state: ResMut<State<PlayerState>>,
) {
}

pub fn spawn_player(mut commands: Commands, inputs: Res<ActionState<Inputs>>) {
    commands.spawn((
        Player,
        Sprite::default(),
        TransformBundle::from_transform(Transform::from_xyz(0., 0., 0.)),
        InputManagerBundle {
            input_map: Inputs::input_map(),
            ..default()
        },
        Velocity::zero(),
        GravityScale(9.),
        LockedAxes::ROTATION_LOCKED_Z,
        RigidBody::Dynamic,
        Ccd { enabled: true },
        Friction::new(0.),
        Collider::cuboid(35. / 2., 60. / 2.),
    ));
}
