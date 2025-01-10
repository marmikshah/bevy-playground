mod brick;
mod bullet;
mod entities;
mod physics;
mod player;
mod resources;
mod systems;

use bevy::prelude::*;
use game::CoreGame;

const WINDOW_HEIGHT: f32 = 720.;
const WINDOW_WIDTH: f32 = 480.;

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
