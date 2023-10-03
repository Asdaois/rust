use core::Actor;
use std::env;

use actors::background_actor::BackgroundActor;
use game::Game;

pub mod actors;
mod core;
mod game;
mod math;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut game = Game::new();
    let background = Box::new(BackgroundActor::new(&mut game.world));

    game.actors.push(background);

    while game.is_running() {
        game.game_loop()
    }
}
