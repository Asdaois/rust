use rltk::GameState;
use specs::{Join, World, WorldExt};

use crate::components::{Position, Renderable};

pub struct State {
    pub ecs: World,
}

impl State {
    pub fn new() -> Self {
        Self { ecs: World::new() }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut rltk::BTerm) {
        ctx.cls();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
