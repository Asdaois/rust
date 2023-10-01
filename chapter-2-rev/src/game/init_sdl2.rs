use sdl2::image::InitFlag;

use super::Game;

impl Game {
    pub(super) fn init_sdl2(&mut self) {
        let Ok(sdl) = sdl2::init() else { return };

        sdl2::image::init(InitFlag::all()).unwrap();

        let window = sdl
            .video()
            .unwrap()
            .window(&self.game_title, self.width, self.height)
            .build()
            .unwrap();

        self.canvas = match window.into_canvas().build() {
            Ok(canvas) => {
                self.texture_creator = Some(canvas.texture_creator());
                Some(canvas)
            }
            Err(_) => panic!("Canvas can't be initialized"),
        };

        self.events = match sdl.event_pump() {
            Ok(e) => Some(e),
            Err(_) => panic!("Events can't be initialized"),
        }
    }
}
