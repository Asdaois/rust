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
        for event in self.world.events.poll_iter() {
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
        self.world
            .canvas
            .set_draw_color(Color::RGBA(0, 0, 255, 255));

        self.world.canvas.clear();

        for actor in self.actors.iter_mut() {
            actor.draw(&mut self.world)
        }

        self.world.canvas.present();
    }
}
