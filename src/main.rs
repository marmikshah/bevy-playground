mod brick;
mod bullet;
mod collision;
mod components;
mod constants;
mod player;
mod resources;
mod systems;

use bevy::prelude::*;
use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use game::CoreGame;

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
            CoreGame,
        ))
        .run();
}
