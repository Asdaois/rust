use crate::actor::Actor;

pub struct Component {
    owner: Box<Actor>,
    update_order: i32,
}

impl Component {
    fn new(owner: Box<Actor>, update_order: i32) -> Self {
        Component {
            owner,
            update_order,
        }
    }
}
