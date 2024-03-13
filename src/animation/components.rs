use bevy::prelude::{Component, Timer};

#[derive(Copy, Clone, Debug)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Clone)]
pub struct Animation {
    pub timer: Timer,
    pub indices: AnimationIndices,
}

impl Animation {
    pub fn set(&mut self, other: &Animation) {
        self.timer.set_duration(other.timer.duration());
        self.indices = other.indices;
    }
}
