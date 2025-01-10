mod entities;
mod physics;
mod resources;
mod sprites;
mod systems;

use bevy::prelude::*;
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
        app.add_plugins((
            sprites::player::PlayerPlugin,
            sprites::bullet::BulletPlugin,
            sprites::brick::BrickPlugin,
            physics::PhysicsManager,
        ))
        .add_systems(Startup, spawn_camera);
    }
}
