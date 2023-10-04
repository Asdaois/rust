use core::Actor;
use std::env;

use actors::{background_actor::BackgroundActor, ship_actor::ShipActor};
use game::Game;
use math::vector_2::Vector2;

pub mod actors;
pub mod components;
mod core;
mod game;
mod math;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut game = Game::new();
    let background = Box::new(BackgroundActor::new());
    game.actors.push(background);

    // let ship = Box::new(ShipActor::new(Vector2 { x: 512., y: 384. }, 1.5, 0.));
    game.init();

    while game.is_running() {
        game.game_loop()
    }
}
