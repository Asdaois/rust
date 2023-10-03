use std::{cell::RefCell, rc::Rc};

use crate::{
    game::{world::Engine, Game},
    math::vector_2::Vector2,
};

pub trait GameLoop {
    fn draw(&mut self, world: &mut Engine);
}

pub trait Actor: GameLoop {
    fn new(world: &mut Engine) -> Self
    where
        Self: Sized;
    fn set_position(&mut self, position: Vector2);
}
trait Component: GameLoop {}
