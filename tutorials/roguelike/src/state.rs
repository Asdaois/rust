use rltk::GameState;
use specs::{Join, RunNow, World, WorldExt};

use crate::{
    components::{Position, Renderable},
    systems::LeftWalker,
};

pub struct State {
    pub ecs: World,
}

impl State {
    pub fn new() -> Self {
        Self { ecs: World::new() }
    }

    pub fn run_systems(&mut self) {
        let mut lw = LeftWalker {};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut rltk::BTerm) {
        ctx.cls();

        self.run_systems();

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
