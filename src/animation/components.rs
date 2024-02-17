use bevy::prelude::{Bundle, Component, Deref, DerefMut, Timer};
use bevy::sprite::SpriteSheetBundle;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Bundle)]
pub struct Animator {
    pub timer: AnimationTimer,
    pub indicies: AnimationIndices,
    pub sprite_sheet: SpriteSheetBundle
}