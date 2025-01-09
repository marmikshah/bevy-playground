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
use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use player::PlayerPlugin;
use systems::spawn_camera;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Shooter".to_string(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            PlayerPlugin {},
            BulletPlugin {},
            BrickPlugin {},
        ))
        .add_systems(Startup, spawn_camera)
        .run();
}
