use bevy::{ecs::query, image::TranscodeFormat, prelude::*};

use crate::{
    bullet::Bullet,
    components::Velocity,
    constants::{BULLET_OFFSET, WINDOW_HEIGHT, WINDOW_WIDTH},
    player::{Player, PlayerBundle},
};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
