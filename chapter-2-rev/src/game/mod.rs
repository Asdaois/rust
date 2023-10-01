use std::{cell::RefCell, collections::HashMap, path::Path, rc::Rc};

use sdl2::{
    image::LoadTexture,
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
    canvas: Option<Canvas<Window>>,
    events: Option<EventPump>,
    texture_creator: Option<TextureCreator<WindowContext>>,
    textures: HashMap<String, Rc<RefCell<Texture>>>,
    actors: Vec<Box<dyn Actor>>,
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

    pub fn load_texture(&mut self, file_name: String) -> Rc<RefCell<Texture>> {
        let Some(tc) = self.texture_creator.as_mut() else {
            panic!("Texture creator don't working, imposible to load textures")
        };

        match self.textures.get(&file_name) {
            Some(t) => return t.clone(),
            None => {
                let t = tc.load_texture(Path::new(&file_name)).unwrap();

                self.textures
                    .insert(file_name.clone(), Rc::new(RefCell::new(t)))
                    .unwrap();

                return self.textures.get(&file_name).unwrap().clone();
            }
        }
    }
}
