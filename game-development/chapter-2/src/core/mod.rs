pub mod component_system;

use crate::{game::world::Engine, math::vector_2::Vector2};

pub type Components = Vec<Box<dyn Component + 'static>>;

pub trait GameLoop {
    fn init(&mut self, engine: &mut Engine);
    fn draw(&mut self, engine: &mut Engine);
    fn update(&mut self, engine: &mut Engine, delta_time: f64);
}

pub trait Actor: GameLoop {
    fn set_position(&mut self, position: Vector2);
}

pub trait Component: GameLoop {}

pub type DrawOrder = usize;
pub type TextureRect = Vector2;

pub trait Sprite {
    fn get_draw_order() -> DrawOrder;
    fn get_texture_rect() -> TextureRect {
        TextureRect {
            ..Default::default()
        }
    }
}
