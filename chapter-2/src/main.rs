use crate::game::Game;

pub mod actor;
pub mod game;
pub mod math;

fn main() {
    let mut game = Box::new(Game::new("Platform".into(), 1024, 768));

    if game.is_running() {
        game.run_loop();
    }
}