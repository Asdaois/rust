use crate::{
    core::{Actor, Component, GameLoop},
    game::world::Engine,
    math::vector_2::Vector2,
};

#[derive(Default)]
pub struct BackgroundActor {
    position: Vector2,
    textures: Vec<String>,
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
        word.draw_texture(self.textures.first().unwrap().clone(), None, None);
    }

    fn update(&mut self, engine: &mut Engine, delta_time: f64) {}
    fn init(&mut self, world: &mut Engine) {
        let texture_file_name = "chapter-2-rev/assets/Farback01.png";
        let _texture = world.load_texture(texture_file_name.into());

        self.textures.push(texture_file_name.into());
    }
}

impl Actor for BackgroundActor {
    fn set_position(&mut self, position: crate::math::vector_2::Vector2) {
        self.position = position
    }

    fn get_components(&self) -> Vec<Box<dyn Component>> {
        vec![]
    }
}
