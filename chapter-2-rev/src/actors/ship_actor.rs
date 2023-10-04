use crate::{
    core::{Actor, GameLoop},
    math::vector_2::Vector2,
};

type Position = Vector2;

pub struct ShipActor {
    position: Position,
    scale: f64,
    rotation: f64,
}

impl ShipActor {
    pub fn new(position: Position, scale: f64, rotation: f64) -> Self
    where
        Self: Sized,
    {
        Self {
            position,
            scale,
            rotation,
        }
    }
}

impl Actor for ShipActor {
    fn set_position(&mut self, position: Vector2) {
        self.position = position;
    }

    fn get_components(&self) -> Vec<Box<dyn crate::core::Component>> {
        todo!()
    }
}

impl GameLoop for ShipActor {
    fn init(&mut self, engine: &mut crate::game::world::Engine) {}

    fn draw(&mut self, engine: &mut crate::game::world::Engine) {}

    fn update(&mut self, engine: &mut crate::game::world::Engine, delta_time: f64) {
        todo!()
    }
}
