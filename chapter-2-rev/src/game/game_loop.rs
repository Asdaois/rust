use sdl2::pixels::Color;

use super::Game;

impl Game {
    pub fn game_loop(&mut self) {
        while self.is_running {
            self.process_input();
            self.update_game();
            self.generate_output();
        }
    }

    fn process_input(&mut self) {
        let Some(e) = self.events.as_mut() else {
            return;
        };

        for event in e.poll_iter() {
            match event {
                sdl2::event::Event::Quit { timestamp: _ } => {
                    self.is_running = false;
                }
                _ => (),
            }
        }
    }

    fn update_game(&self) {}

    fn generate_output(&mut self) {
        let Some(canvas) = self.canvas.as_mut() else {
            return;
        };

        canvas.set_draw_color(Color::RGBA(0, 0, 255, 255));

        canvas.clear();

        for actor in self.actors.iter_mut() {
            actor.draw()
        }

        canvas.present();
    }
}
