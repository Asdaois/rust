use super::{Component, Components, GameLoop};

struct ComponentSystem {
    components: Components,
}

impl GameLoop for ComponentSystem {
    fn init(&mut self, engine: &mut crate::game::world::Engine) {}

    fn draw(&mut self, engine: &mut crate::game::world::Engine) {
        for component in self.components.iter_mut() {
            component.draw(engine)
        }
    }

    fn update(&mut self, engine: &mut crate::game::world::Engine, delta_time: f64) {
        for component in self.components.iter_mut() {
            component.update(engine, delta_time)
        }
    }
}

impl ComponentSystem {
    fn new(components: Components) -> Self {
        Self { components }
    }

    fn add(&mut self, component: impl Component + 'static) {
        let components = &mut self.components;
        components.push(Box::new(component));
    }
}
