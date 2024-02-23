use bevy::prelude::Component;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Init;

macro_rules! query_guard {
    ($query:expr) => {
        if let Ok(result) = $query {
            result
        } else {
            return;
        }
    };
    ($($query:expr),+) => {
        ($(query_guard!($query),)+)
    }
}

macro_rules! on_enter {
    ($commands:ident, $entity:expr, $init:ident $(, $logic:tt)?) => {
        if $init {
            $( $logic )?
            $commands
                .entity($entity)
                .remove::<Init>();
        }
    };
}

macro_rules! change_state {
    ($commands:ident, $entity:expr, $in_state:ty, $next_state:expr $(, $transition_logic:tt)?) => {
        $commands
            .entity($entity)
            .remove::<$in_state>()
            .insert(($next_state, Init));
        $( $transition_logic )?
    };
}

pub(crate) use {change_state, on_enter, query_guard};
