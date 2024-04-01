use bevy::prelude::*;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct JustEntered;

pub fn change_state<R: Bundle, I: Bundle>(
    commands: &mut Commands, 
    entity: Entity,
    state: I
) {   
    commands.entity(entity).insert((state, JustEntered)).remove::<R>();
}