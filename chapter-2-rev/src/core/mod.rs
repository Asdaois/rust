use crate::{game::world::Engine, math::vector_2::Vector2};

pub trait GameLoop {
    fn init(&mut self, engine: &mut Engine);
    fn draw(&mut self, engine: &mut Engine);
    fn update(&mut self, engine: &mut Engine, delta_time: f64);
}

pub trait Actor: GameLoop {
    fn set_position(&mut self, position: Vector2);
    fn update_components(&mut self, engine: &mut Engine, delta_time: f64) {
        for component in self.get_components().iter_mut() {
            component.update(engine, delta_time)
        }
    }
    fn get_components(&self) -> Vec<Box<dyn Component>>;
    fn add_component(&mut self, component: Box<dyn Component>) {
        self.get_components().push(component);
    }
}
pub trait Component: GameLoop {}
