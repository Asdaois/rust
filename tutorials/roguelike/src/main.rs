use components::{Position, Renderable};
use specs::{Builder, WorldExt};

use crate::{components::LeftMover, state::State};
mod components;
mod entities;
mod state;
mod systems;

fn main() -> rltk::BError {
    let mut gs = State::new(); // Global State
    gs.ecs.register::<components::Position>();
    gs.ecs.register::<components::Renderable>();
    gs.ecs.register::<components::LeftMover>();
    gs.ecs.register::<entities::Player>();

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: rltk::RGB::named(rltk::YELLOW),
            bg: rltk::RGB::named(rltk::BLACK),
        })
        .with(entities::Player {})
        .build();

    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: rltk::to_cp437('☺'),
                fg: rltk::RGB::named(rltk::RED),
                bg: rltk::RGB::named(rltk::BLACK),
            })
            .with(LeftMover {})
            .build();
    }

    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("Rogue like tutorial")
        .build()
        .expect("Failed to build terminal");

    rltk::main_loop(context, gs)
}
