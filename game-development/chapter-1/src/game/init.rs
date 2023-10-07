use super::{Game, Vector2, HEIGHT, WIDTH};

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
                    paddle_position: Vector2 {
                        x: 5.,
                        y: HEIGHT as f64 / 2.0 - 25.,
                    },
                    ball_position: Vector2 {
                        x: WIDTH as f64 / 2.0,
                        y: HEIGHT as f64 / 2.0 - 7.5,
                    },
                    ball_velocity: Vector2 { x: 200., y: 235. },
                    ball_direction: true,
                    ticks_count: 0,
                    paddle_direction: 0,
                };
            }
            Err(message) => {
                panic!("Unable to initialize SDL: {}", message);
            }
        }
    }
}
