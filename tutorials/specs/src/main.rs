use specs::{storage, Component, ReadStorage, RunNow, System, VecStorage};

#[derive(Debug, Component)]
#[storage(VecStorage)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, Component)]
#[storage(VecStorage)]
struct Velocity {
    x: f32,
    y: f32,
}

struct MyWorld;

impl<'a> System<'a> for MyWorld {
    type SystemData = (ReadStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        for position in data.join() {
            println!("Hello, {:?}", position);
        }
    }
}

fn main() {
    use specs::{Builder, World, WorldExt};

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();

    let ball = world
        .create_entity()
        .with(Position { x: 4., y: 7. })
        .build();

    let mut my_world = MyWorld;
    my_world.run_now(&world);

    world.maintain();
}
