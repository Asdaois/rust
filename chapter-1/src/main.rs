use game::Game;

mod game;

fn main() {
    let mut game = Game::init();

    if game.is_running() {
        game.run_loop();
    }

    unsafe { game.shut_down() };
}
