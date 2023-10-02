use std::{cell::RefCell, rc::Rc};



use crate::{game::Game, math::vector_2::Vector2};

pub trait GameLoop {
    fn draw(&mut self);
}

pub trait Actor: GameLoop {
    fn new(game: Rc<RefCell<Game>>) -> Self
    where
        Self: Sized;
    fn set_position(&mut self, position: Vector2);
}
trait Component: GameLoop {}
