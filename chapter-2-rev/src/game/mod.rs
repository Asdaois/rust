use std::{collections::HashMap, path::Path, rc::Rc};

use sdl2::{
    image::LoadTexture,
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
    EventPump,
};

use crate::core::Actor;

mod game_loop;
mod init_sdl2;

#[derive(Default)]
pub struct Game {
    game_title: String,
    width: u32,
    height: u32,
    is_running: bool,
    pub canvas: Option<Canvas<Window>>,
    events: Option<EventPump>,
    texture_creator: Option<TextureCreator<WindowContext>>,
    textures: HashMap<String, Rc<Texture>>,
    pub actors: Vec<Box<dyn Actor>>,
}

impl Game {
    /// Creates a new [`Game`].
    pub fn new() -> Self {
        let mut game = Game {
            game_title: "ECS".into(),
            width: 1024,
            height: 768,
            ..Default::default()
        };
        game.init_sdl2();

        game.is_running = true;

        game
    }

    pub(crate) fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn load_texture(&mut self, file_name: String) -> Rc<Texture> {
        let Some(tc) = self.texture_creator.as_mut() else {
            panic!("Texture creator don't working, imposible to load textures")
        };

        match self.textures.get(&file_name) {
            Some(t) => return t.clone(),
            None => {
                let texture = tc
                    .load_texture(Path::new(&file_name))
                    .expect("Texture don't loaded");

                self.textures.insert(file_name.clone(), Rc::new(texture));

                return self.textures.get(&file_name).unwrap().clone();
            }
        }
    }

    pub fn draw_texture(&mut self, file_name: String, src: Option<Rect>, dst: Option<Rect>) {
        let texture = self.load_texture(file_name);

        match self.canvas {
            Some(ref mut c) => {
                let _ = c.copy(texture.as_ref(), src, dst);
            }
            None => todo!(),
        }
    }
}
