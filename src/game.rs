mod init;

use sdl2::{
    keyboard::Scancode,
    pixels::Color,
    render::Canvas,
    sys::{SDL_DestroyWindow, SDL_Quit},
    video::Window,
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
        self.canvas.present();
    }
}
