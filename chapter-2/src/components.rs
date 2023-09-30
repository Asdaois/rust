pub mod background_sprite_component;

use sdl2::render::Texture;

use crate::actor::Actor;

pub trait Component {
    /// .
    fn new(owner: Box<Actor>, draw_order: u32) -> Self
    where
        Self: Sized;
    fn update(&self, delta_time: f64);
    fn update_order(&self) -> u32;
}

trait SpriteComponent: Component {
    fn draw_order() -> i32;
    fn texture_height() -> i32;
    fn texture_width() -> i32;
    fn draw();
    fn set_texture(texture: Texture);
}
