use std::rc::Rc;

use crate::components::Component;

use super::Actor;

impl Actor {
    fn add_component(&self, component: Box<impl Component>) {}
    fn remove_component<C>(&self, component: Box<impl Component>) {}
}
