use sdl2::render::Texture;

use crate::core::{Component, GameLoop};

struct SpriteComponent {
    draw_order: i64,
    textures: Vec<String>,
}

impl GameLoop for SpriteComponent {
    fn draw(&mut self, engine: &mut crate::game::engine::Engine) {
        todo!()
    }

    fn update(&mut self, engine: &mut crate::game::engine::Engine, delta_time: f64) {
        todo!()
    }

    fn init(&mut self, engine: &mut crate::game::engine::Engine) {
        for texture in self.textures.iter() {
            engine.load_texture(texture.into());
        }
    }
}

impl Component for SpriteComponent {}
