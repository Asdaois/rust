use std::{borrow::BorrowMut, cell::RefCell, ops::Deref, rc::Rc};

use sdl2::render::Texture;

use crate::{core::Actor, game::Game, math::vector_2::Vector2};

#[derive(Default)]
struct BackgroundActor {
    position: Vector2,
    game: Rc<RefCell<Game>>,
    textures: Vec<Rc<RefCell<Texture>>>,
}

impl BackgroundActor {
    fn init(&mut self) {
        let texture = self
            .game
            .as_ref()
            .borrow_mut()
            .load_texture("assets/Farback01.png".into());

        self.textures.push(Rc::clone(&texture));
    }
}

impl Actor for BackgroundActor {
    fn new(game: Rc<RefCell<Game>>) -> Self {
        Self {
            game,
            ..Default::default()
        }
    }
    fn set_position(&mut self, position: crate::math::vector_2::Vector2) {
        self.position = position
    }
}
