use crate::{game::world::Engine, math::vector_2::Vector2};

pub trait GameLoop {
    fn draw(&mut self, engine: &mut Engine);
}

pub trait Actor: GameLoop {
    fn new(engine: &mut Engine) -> Self
    where
        Self: Sized;
    fn set_position(&mut self, position: Vector2);
}
trait Component: GameLoop {}
