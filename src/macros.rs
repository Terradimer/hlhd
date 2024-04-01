use bevy::prelude::Component;

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
            $commands
                .entity($entity)
                .remove::<JustEntered>();
            $( $logic )?
        }
    };
}

pub(crate) use {on_enter, query_guard};
