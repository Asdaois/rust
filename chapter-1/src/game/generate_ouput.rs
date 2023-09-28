use sdl2::{pixels::Color, rect::Rect};

use super::{Game, HEIGHT, THICKNESS, WIDTH};

impl Game {
    pub fn generate_output(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 255, 255));

        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));

        self.draw_walls();

        self.draw_paddle();

        self.draw_ball();

        self.canvas.present();
    }

    fn draw_ball(&mut self) {
        self.canvas
            .fill_rect(Rect::new(
                self.ball_position.x as i32,
                self.ball_position.y as i32,
                THICKNESS,
                THICKNESS,
            ))
            .unwrap();
    }

    fn draw_paddle(&mut self) {
        self.canvas
            .fill_rect(Rect::new(
                self.paddle_position.x as i32,
                self.paddle_position.y as i32,
                THICKNESS,
                50,
            ))
            .unwrap();
    }

    fn draw_walls(&mut self) {
        self.canvas
            .fill_rect(Rect::new(0, 0, WIDTH, THICKNESS))
            .unwrap();

        self.canvas
            .fill_rect(Rect::new(0, (HEIGHT - THICKNESS) as i32, WIDTH, THICKNESS))
            .unwrap();

        self.canvas
            .fill_rect(Rect::new((WIDTH - THICKNESS) as i32, 0, THICKNESS, HEIGHT))
            .unwrap();
    }
}
