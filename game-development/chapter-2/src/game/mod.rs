use crate::core::Actor;

use self::engine::Engine;

pub mod engine;
mod game_loop;

pub struct Game {
    is_running: bool,
    width: u32,
    height: u32,
    game_title: String,
    pub engine: Engine,

    pub actors: Vec<Box<dyn Actor>>,
}

impl Game {
    /// Creates a new [`Game`].
    pub fn new() -> Self {
        let mut game = Game {
            width: 1024,
            height: 768,
            game_title: "ECS".into(),
            engine: Engine::new("ECS".into(), 1024, 768),
            is_running: false,
            actors: vec![],
        };

        game
    }

    pub fn init(&mut self) {
        for actor in self.actors.iter_mut() {
            actor.init(&mut self.engine);
        }
        self.is_running = true;
    }

    pub(crate) fn is_running(&self) -> bool {
        self.is_running
    }
}
