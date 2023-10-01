use sdl2::{
    render::{Canvas, TextureCreator},
    video::{Window, WindowContext},
    EventPump,
};

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
}
