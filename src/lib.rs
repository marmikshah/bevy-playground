mod brick;
mod bullet;
mod collision;
mod components;
mod constants;
mod player;
mod resources;
mod systems;

use bevy::prelude::*;
use brick::BrickPlugin;
use bullet::BulletPlugin;
use collision::CollisionEngine;
use player::PlayerPlugin;
use systems::spawn_camera;

/**
 * CoreGame plugin only creates resources and systems that
 * are common across different platforms.
 * Mainly Control/Gamepad will be platform specific,
 * i.e. either via Touch, Kayboard of Controller.
 */
pub struct CoreGame;

impl Plugin for CoreGame {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, BulletPlugin, BrickPlugin, CollisionEngine))
            .add_systems(Startup, spawn_camera);
    }
}
