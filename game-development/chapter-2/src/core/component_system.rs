use super::{Component, Components, GameLoop};

pub struct ComponentSystem {
    pub components: Components,
}

impl Default for ComponentSystem {
    fn default() -> Self {
        Self {
            components: Default::default(),
        }
    }
}

impl GameLoop for ComponentSystem {
    fn init(&mut self, engine: &mut crate::game::engine::Engine) {}

    fn draw(&mut self, engine: &mut crate::game::engine::Engine) {
        for component in self.components.iter_mut() {
            component.draw(engine)
        }
    }

    fn update(&mut self, engine: &mut crate::game::engine::Engine, delta_time: f64) {
        for component in self.components.iter_mut() {
            component.update(engine, delta_time)
        }
    }
}

impl ComponentSystem {
    fn new(components: Components) -> Self {
        Self { components }
    }

    pub fn add(&mut self, component: impl Component + 'static) {
        self.components.push(Box::new(component));
    }
}
