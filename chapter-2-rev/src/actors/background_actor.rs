use crate::{
    core::{Actor, GameLoop},
    game::world::Engine,
    math::vector_2::Vector2,
};

#[derive(Default)]
pub struct BackgroundActor {
    position: Vector2,
    textures: Vec<String>,
}

impl BackgroundActor {
    fn init(&mut self, world: &mut Engine) {
        let texture_file_name = "chapter-2-rev/assets/Farback01.png";
        let _texture = world.load_texture(texture_file_name.into());

        self.textures.push(texture_file_name.into());
    }
}

impl GameLoop for BackgroundActor {
    fn draw(&mut self, word: &mut Engine) {
        word.draw_texture(self.textures.first().unwrap().clone(), None, None);
    }
}

impl Actor for BackgroundActor {
    fn new(worl: &mut Engine) -> Self {
        let mut ba = Self {
            ..Default::default()
        };

        ba.init(worl);

        ba
    }
    fn set_position(&mut self, position: crate::math::vector_2::Vector2) {
        self.position = position
    }
}
