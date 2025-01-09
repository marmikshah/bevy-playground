mod brick;
mod bullet;
mod components;
mod constants;
mod player;
mod resources;
mod systems;

use bevy::prelude::*;
use brick::BrickPlugin;
use bullet::BulletPlugin;
use player::PlayerPlugin;
use systems::spawn_camera;

pub struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        App::new()
            .add_plugins((
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        mode: bevy::window::WindowMode::BorderlessFullscreen(
                            (MonitorSelection::Primary),
                        ),
                        recognize_rotation_gesture: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                PlayerPlugin,
                BulletPlugin,
                BrickPlugin,
            ))
            .add_systems(Startup, spawn_camera)
            .run();
    }
}
