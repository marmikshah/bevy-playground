use bevy::prelude::*;
use game::Game;

#[bevy_main]
fn main() {
    let mut app = App::new();
    app.add_plugins(Game).run();
}
