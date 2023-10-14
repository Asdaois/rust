use crate::core::*;

pub struct BackgroundComponent {
    textures: Vec<String>,
    draw_order: DrawOrder,
}

impl BackgroundComponent {
    pub fn new(draw_order: Option<DrawOrder>) -> Self {
        Self {
            textures: vec![],
            draw_order: draw_order.unwrap_or(10),
        }
    }

    pub fn add_texture(&mut self, texture: String) {
        self.textures.push(texture.into());
    }
}

impl GameLoop for BackgroundComponent {
    fn init(&mut self, engine: &mut crate::game::engine::Engine) {
        for texture in self.textures.iter() {
            engine.load_texture(texture.into());
        }
    }

    fn draw(&mut self, engine: &mut crate::game::engine::Engine) {
        engine.draw_texture(self.textures.first().unwrap().clone(), None, None);
    }

    fn update(&mut self, engine: &mut crate::game::engine::Engine, delta_time: f64) {
        todo!()
    }
}
impl Component for BackgroundComponent {}

impl Sprite for BackgroundComponent {
    fn get_draw_order() -> DrawOrder {
        todo!()
    }
}
