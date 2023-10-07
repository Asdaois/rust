use specs::{Join, ReadStorage, System, WriteStorage};

use crate::components::{LeftMover, Position};

pub struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut position): Self::SystemData) {
        for (_lefty, position) in (&lefty, &mut position).join() {
            position.x -= 1;

            if position.x < 0 {
                position.x = 79;
            }
        }
    }
}
