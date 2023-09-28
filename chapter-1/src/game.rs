mod init;
static THICKNESS: u32 = 15;
static PADDLE_H: f32 = 100.0;
static WIDTH: u32 = 480;
static HEIGHT: u32 = 480;

use sdl2::{
    keyboard::Scancode, pixels::Color, rect::Rect, render::Canvas, sys::SDL_Quit, video::Window,
    Sdl,
};
pub struct Game {
    is_running: bool,
    canvas: Canvas<Window>,
    sdl_context: Sdl,
}

impl Game {
    pub fn is_running(&self) -> bool {
        self.is_running
    }
    /// Runs the game loop until the [`Game`] is over
    pub fn run_loop(&mut self) {
        while self.is_running {
            self.process_input();
            self.update_game();
            self.generate_output();
        }
    }
    /// Shutdown the [`Game`]
    pub unsafe fn shut_down(&self) {
        SDL_Quit();
    }

    fn process_input(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { timestamp: _ } => {
                    self.is_running = false;
                }
                _ => (),
            }
        }

        let keyboard = event_pump.keyboard_state();

        if keyboard.is_scancode_pressed(Scancode::Escape) {
            self.is_running = false;
        }
    }
    fn update_game(&self) {}
    fn generate_output(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 255, 255));

        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));

        self.canvas
            .fill_rect(Rect::new(0, 0, WIDTH, THICKNESS))
            .unwrap();

        self.canvas
            .fill_rect(Rect::new(0, (HEIGHT - THICKNESS) as i32, WIDTH, THICKNESS))
            .unwrap();

        self.canvas
            .fill_rect(Rect::new((WIDTH - THICKNESS) as i32, 0, THICKNESS, HEIGHT))
            .unwrap();

        self.canvas
            .fill_rect(Rect::new(
                (THICKNESS / 2).try_into().unwrap(),
                (HEIGHT / 2).try_into().unwrap(),
                THICKNESS,
                50,
            ))
            .unwrap();

        self.canvas
            .fill_rect(Rect::new(240, 240, THICKNESS, THICKNESS))
            .unwrap();

        self.canvas.present();
    }
}
