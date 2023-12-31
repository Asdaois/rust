mod generate_ouput;
mod init;
pub static THICKNESS: u32 = 15;
pub static PADDLE_H: f32 = 100.0;
pub static WIDTH: u32 = 480;
pub static HEIGHT: u32 = 480;

use std::time::Duration;

use sdl2::{keyboard::Scancode, render::Canvas, sys::SDL_Quit, video::Window, Sdl};

pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

pub struct Game {
    is_running: bool,
    canvas: Canvas<Window>,
    sdl_context: Sdl,

    paddle_position: Vector2,
    paddle_direction: i32,
    ball_position: Vector2,
    ball_velocity: Vector2,
    ball_direction: bool,
    ticks_count: u32,
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

        self.paddle_direction = 0;
        if keyboard.is_scancode_pressed(Scancode::W) {
            self.paddle_direction -= 1;
        }
        if keyboard.is_scancode_pressed(Scancode::S) {
            self.paddle_direction += 1;
        }
    }

    fn update_game(&mut self) {
        let ticks = self.sdl_context.timer().unwrap().ticks();
        let ticks_from_previous = (ticks - self.ticks_count) as f64;
        std::thread::sleep(Duration::from_millis(16));

        let mut delta_time: f64 = ticks_from_previous / 100.0;

        if delta_time > 0.05 {
            delta_time = 0.05;
        }

        if self.paddle_direction != 0 {
            self.paddle_position.y += self.paddle_direction as f64 * 300. * delta_time;

            let top_border = PADDLE_H as f64 / 2. - 40.;
            let bot_border = HEIGHT as f64 - PADDLE_H as f64 / 2. - THICKNESS as f64;

            if self.paddle_position.y < top_border {
                self.paddle_position.y = top_border;
            } else if self.paddle_position.y > bot_border {
                self.paddle_position.y = bot_border;
            }
        }

        self.ball_position.x += self.ball_velocity.x * delta_time;
        self.ball_position.y += self.ball_velocity.y * delta_time;

        let diff = (self.paddle_position.y - self.ball_position.y).abs();
        if diff <= PADDLE_H as f64 / 2. 
		    // We are in the correct x-position
		    && self.ball_position.x <= 25. && self.ball_position.x >= 20. 
		    // The ball is moving to the left
		    && self.ball_velocity.x < 0.
        {
            self.ball_velocity.x *= -1.;
        } else if self.ball_position.x < 0. {
            self.is_running = false;
        } else if self.ball_position.x >= (WIDTH - THICKNESS) as f64
        && self.ball_velocity.x > 0. {
            self.ball_velocity.x *= -1.;
        }

        if self.ball_position.y <= THICKNESS as f64 && self.ball_velocity.y < 0. {
            self.ball_velocity.y *= -1.;
        }
        else if self.ball_position.y >= (HEIGHT - THICKNESS) as f64 
        && self.ball_velocity.y > 0.
	    {
		    self.ball_velocity.y *= -1.;
	    }

        self.ticks_count = ticks;
    }
}
