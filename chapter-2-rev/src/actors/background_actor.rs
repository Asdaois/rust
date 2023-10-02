use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use crate::{
    core::{Actor, GameLoop},
    game::Game,
    math::vector_2::Vector2,
};

#[derive(Default)]
pub struct BackgroundActor {
    position: Vector2,
    game: Rc<RefCell<Game>>,
    textures: Vec<String>,
}

impl BackgroundActor {
    fn init(&mut self) {
        let texture_name = "chapter-2-rev/assets/Farback01.png";
        let _texture = self
            .game
            .as_ref()
            .borrow_mut()
            .load_texture(texture_name.into());

        self.textures.push(texture_name.into());
    }
}

impl GameLoop for BackgroundActor {
    fn draw(&mut self) {
        let mut game = self.game.as_ref().borrow_mut();

        game.draw_texture(self.textures.first().unwrap().clone(), None, None);
    }
}

impl Actor for BackgroundActor {
    fn new(game: Rc<RefCell<Game>>) -> Self {
        let mut ba = Self {
            game,
            ..Default::default()
        };

        ba.init();

        ba
    }
    fn set_position(&mut self, position: crate::math::vector_2::Vector2) {
        self.position = position
    }
}
