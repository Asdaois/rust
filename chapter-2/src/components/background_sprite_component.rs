use std::{collections::HashMap, rc::Rc};

use sdl2::render::Texture;

use crate::{actor::Actor, math::vector_2::Vector2};

use super::{Component, SpriteComponent};

struct BackgroundTexture {
    texture: Rc<Texture>,
    offset: Vector2,
}

struct BackgroundSpriteComponent {
    background_textures: Vec<BackgroundTexture>,
    scroll_speed: f64,
    screen_size: Vector2,
    owner: Box<Actor>,
    draw_order: u32,
}

impl Component for BackgroundSpriteComponent {
    fn new(owner: Box<crate::actor::Actor>, draw_order: u32) -> Self
    where
        Self: Sized,
    {
        Self {
            background_textures: vec![],
            scroll_speed: 0.,
            screen_size: Vector2 {},
            owner,
            draw_order,
        }
    }

    fn update(&self, delta_time: f64) {}

    fn update_order(&self) -> u32 {
        todo!()
    }
}

impl SpriteComponent for BackgroundSpriteComponent {
    fn draw_order() -> i32 {
        todo!()
    }

    fn texture_height() -> i32 {
        todo!()
    }

    fn texture_width() -> i32 {
        todo!()
    }

    fn draw() {
        todo!()
    }

    fn set_texture(texture: Texture) {
        todo!()
    }
}
