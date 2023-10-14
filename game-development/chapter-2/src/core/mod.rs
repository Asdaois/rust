use crate::{game::world::Engine, math::vector_2::Vector2};

pub type Components = Vec<Box<dyn Component>>;

pub trait GameLoop {
    fn init(&mut self, engine: &mut Engine);
    fn draw(&mut self, engine: &mut Engine);
    fn update(&mut self, engine: &mut Engine, delta_time: f64);
}

pub trait ComponentSystem {
    fn update(&mut self, engine: &mut Engine, delta_time: f64) {
        for component in self.get().iter_mut() {
            component.update(engine, delta_time)
        }
    }
    fn get(&self) -> Components;
    fn add(&mut self, component: impl Component + 'static) {
        let components = &mut self.get();
        components.push(Box::new(component));
    }
}

pub trait Actor: GameLoop {
    fn set_position(&mut self, position: Vector2);
}

impl ComponentSystem for dyn Actor {
    fn get(&self) -> Components {
        vec![]
    }
}

pub trait Component: GameLoop {}
