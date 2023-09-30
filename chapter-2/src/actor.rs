use std::rc::Rc;

use crate::{components::Component, game::Game};

use self::{state::State, transform::Transform};

mod components;
pub mod state;
pub mod transform;
mod update;

pub struct Actor {
    /// Actor's state
    pub state: State,
    pub transform: Transform,
    pub components: Vec<Box<dyn Component>>,
    game: Rc<Game>,
}

impl Actor {
    pub fn new(game: Rc<Game>) {}
}
