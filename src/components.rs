use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Direction(pub i8);

#[derive(Component)]
pub struct Speed(pub f32);

