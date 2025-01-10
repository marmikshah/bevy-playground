use bevy::prelude::*;
use game::CoreGame;

#[bevy_main]
fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                mode: bevy::window::WindowMode::BorderlessFullscreen((MonitorSelection::Primary)),
                recognize_rotation_gesture: true,
                ..Default::default()
            }),
            ..Default::default()
        }),
        CoreGame,
    )).run();
}
