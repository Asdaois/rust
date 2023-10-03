use crate::core::Actor;

use self::world::Engine;

mod game_loop;
pub mod world;

pub struct Game {
    is_running: bool,
    width: u32,
    height: u32,
    game_title: String,
    pub world: Engine,

    pub actors: Vec<Box<dyn Actor>>,
}

impl Game {
    /// Creates a new [`Game`].
    pub fn new() -> Self {
        let mut game = Game {
            width: 1024,
            height: 768,
            game_title: "ECS".into(),
            world: Engine::new("ECS".into(), 1024, 768),
            is_running: false,
            actors: vec![],
        };

        game.is_running = true;

        game
    }

    pub(crate) fn is_running(&self) -> bool {
        self.is_running
    }
}
