use super::Game;

impl Game {
    pub fn init() -> Game {
        let sdl_context = sdl2::init();
        match sdl_context {
            Ok(sdl) => {
                let video_subsystem = sdl.video().unwrap();
                let window = video_subsystem.window("Game", 480, 480).build().unwrap();
                let canvas = window.into_canvas().build().unwrap();

                return Game {
                    canvas,
                    is_running: true,
                    sdl_context: sdl,
                };
            }
            Err(message) => {
                panic!("Unable to initialize SDL: {}", message);
            }
        }
    }
}
