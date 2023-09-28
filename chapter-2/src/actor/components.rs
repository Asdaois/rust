use std::rc::Rc;

use super::Actor;

pub struct Component {
    owner: Rc<Actor>,
    update_order: i32,
}

impl Component {
    fn new(owner: Rc<Actor>, update_order: i32) -> Self {
        Component {
            owner,
            update_order,
        }
    }
}

impl Actor {
    fn add_component(&self, component: Component) {}
    fn remove_component(&self, component: Component) {}
}
