use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct HealthPoints(pub i32);

#[derive(Component)]
pub struct Power(pub i32);
