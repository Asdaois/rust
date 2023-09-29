pub mod sprite_component;

use crate::actor::Actor;

pub trait Component {
    /// .
    fn new(owner: Box<Actor>, draw_order: u32) -> Self
    where
        Self: Sized;
    fn update(&self, delta_time: f64);
    fn update_order(&self) -> u32;
}
