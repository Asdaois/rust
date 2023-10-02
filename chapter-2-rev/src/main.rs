use core::Actor;
use std::{cell::RefCell, env, rc::Rc};

use actors::background_actor::BackgroundActor;
use game::Game;

pub mod actors;
mod core;
mod game;
mod math;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let game = Rc::new(RefCell::new(Game::new()));
    let background = Box::new(BackgroundActor::new(game.clone()));

    game.as_ref().borrow_mut().actors.push(background);

    while game.as_ref().borrow().is_running() {
        game.as_ref().borrow_mut().game_loop()
    }
}
