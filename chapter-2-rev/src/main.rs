use game::Game;

pub mod actors;
mod core;
mod game;

fn main() {
    let mut game = Game::new();

    while game.is_running() {
        game.game_loop()
    }
}
