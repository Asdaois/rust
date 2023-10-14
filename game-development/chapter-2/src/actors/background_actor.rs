use crate::{
    components::{self, background_component::BackgroundComponent},
    core::{Actor, Component, Components, GameLoop},
    game::world::Engine,
    math::vector_2::Vector2,
};

#[derive(Default)]
pub struct BackgroundActor {
    position: Vector2,
    textures: Vec<String>,
    components: Components,
}

impl BackgroundActor {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl GameLoop for BackgroundActor {
    fn draw(&mut self, word: &mut Engine) {
        for component in self.components.iter_mut() {
            component.draw(word);
        }
    }

    fn update(&mut self, engine: &mut Engine, delta_time: f64) {}
    fn init(&mut self, world: &mut Engine) {
        let mut component = BackgroundComponent::new(None);
        component.add_texture("assets/Farback01.png".into());
        component.add_texture("assets/Farback02.png".into());
    }
}

impl Actor for BackgroundActor {
    fn set_position(&mut self, position: crate::math::vector_2::Vector2) {
        self.position = position
    }
}
