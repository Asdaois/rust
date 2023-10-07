use std::{collections::HashMap, path::Path, rc::Rc, time::Duration};

use sdl2::{
    image::{InitFlag, LoadTexture},
    pixels::Color,
    render::{Canvas, Texture, TextureCreator},
    video::{Window, WindowContext},
};

use crate::actor::{self, Actor};

pub struct Game {
    subsystem_timer: sdl2::TimerSubsystem,
    input_event_pump: sdl2::EventPump,
    texture_creator: TextureCreator<WindowContext>,
    canvas: Canvas<Window>,
    actors: Vec<Actor>,
    pending_actors: Vec<Actor>,
    ticks_count: u32,
    is_running: bool,
    textures: HashMap<String, Rc<Texture>>,
}

impl Game {
    pub fn is_running(&self) -> bool {
        self.is_running
    }
    pub fn new(game_title: String, width: u32, height: u32) -> Self {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        sdl2::image::init(InitFlag::all()).unwrap();

        let window = video_subsystem
            .window(&game_title, width, height)
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();

        Game {
            canvas,
            subsystem_timer: sdl.timer().unwrap(),
            actors: vec![],
            pending_actors: vec![],
            ticks_count: 0,
            is_running: true,
            input_event_pump: sdl.event_pump().unwrap(),
            texture_creator,
            textures: Default::default(),
        }
    }
    pub fn run_loop(&mut self) {
        while self.is_running {
            self.process_input();
            self.update_game();
            self.generate_output();
        }
    }

    pub fn update_game(&mut self) {
        let ticks = self.subsystem_timer.ticks();
        let ticks_from_previous = (ticks - self.ticks_count) as f64;
        std::thread::sleep(Duration::from_millis(16));
        let delta_time: f64 = ticks_from_previous / 100.0;

        let mut updating_actors = true;

        for actor in &self.actors {
            actor.update(delta_time);
        }

        updating_actors = false;

        self.actors.append(&mut self.pending_actors);

        let mut dead_actors: Vec<&mut Actor> = self
            .actors
            .iter_mut()
            .filter(|actor| actor.state == actor::state::State::Dead)
            .collect::<_>();

        dead_actors.clear();

        self.ticks_count = ticks;
    }

    fn process_input(&mut self) {
        for event in self.input_event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { timestamp: _ } => {
                    self.is_running = false;
                }
                _ => (),
            }
        }
    }

    fn generate_output(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 255, 255));

        self.canvas.clear();

        self.canvas.present();
    }

    pub fn load_png(mut self, filename: String) -> Rc<sdl2::render::Texture> {
        if let Some(t) = self.textures.get(&filename) {
            return t.clone();
        } else {
            let t = self
                .texture_creator
                .load_texture(Path::new(&filename))
                .unwrap();

            self.textures.insert(filename.clone(), Rc::new(t)).unwrap();

            return self.textures.get(&filename).unwrap().clone();
        }
    }
}
