use std::rc::Rc;

use crate::game::Game;

use self::{components::Component, state::State, transform::Transform};

mod components;
mod state;
pub mod transform;
mod update;

struct Actor {
    /// Actor's state
    pub state: State,
    pub transform: Transform,
    pub components: Vec<Component>,
    game: Rc<Game>,
}

impl Actor {
    pub fn new(game: Rc<Game>) {}
}
