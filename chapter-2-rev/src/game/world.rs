use std::collections::HashMap;

use sdl2::{
    image::{InitFlag, LoadTexture},
    rect::Rect,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
    EventPump,
};

/// [`Engine`].
/// Wrapper round sdl2 engine modules
/// contains a simple texture manager to avoid reloading a created texture
pub struct Engine {
    pub canvas: Canvas<Window>,
    pub events: EventPump,
    pub texture_creator: TextureCreator<WindowContext>,
    pub textures: HashMap<String, Texture>,
}

impl Engine {
    /// Creates a new [`Engine`].
    ///
    /// # Panics
    ///
    /// Panics if .
    pub fn new(game_title: String, width: u32, height: u32) -> Self {
        let sdl = sdl2::init().expect("Fail to initialize SDL2");

        sdl2::image::init(InitFlag::all()).unwrap();

        let window = sdl
            .video()
            .unwrap()
            .window(&game_title, width, height)
            .build()
            .unwrap();

        let (canvas, texture_creator) = match window.into_canvas().build() {
            Ok(canvas) => {
                let texture_creator = canvas.texture_creator();
                (canvas, texture_creator)
            }
            Err(_) => panic!("Canvas can't be initialized"),
        };

        let events = match sdl.event_pump() {
            Ok(e) => e,
            Err(_) => panic!("Events can't be initialized"),
        };

        Self {
            canvas,
            events,
            texture_creator,
            textures: HashMap::new(),
        }
    }

    pub fn load_texture(&mut self, file_name: String) {
        if self.textures.contains_key(&file_name) {
            return;
        }

        let texture = self
            .texture_creator
            .load_texture(&file_name)
            .expect("Texture don't loaded");

        let file_name_copy = file_name.clone();
        self.textures.insert(file_name_copy, texture);
    }

    pub fn draw_texture(&mut self, file_name: String, src: Option<Rect>, dst: Option<Rect>) {
        self.load_texture(file_name.clone());

        let texture = self.textures.get(&file_name);

        self.canvas
            .copy(texture.unwrap(), src, dst)
            .expect("Texture can be draw");
    }
}
