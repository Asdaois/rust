use specs::{Component, DispatcherBuilder, Read, ReadStorage, System, VecStorage, WriteStorage};

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
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;

        for position in data.join() {
            println!("Hello, {:?}", position);
        }
    }
}

struct UpdatePosition;

impl<'a> System<'a> for UpdatePosition {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (delta_time, vel, mut pos): Self::SystemData) {
        use specs::Join;

        for (velocity, position) in (&vel, &mut pos).join() {
            position.x += velocity.x * delta_time.0;
            position.y += velocity.y * delta_time.0;
            println!("Update velocity");
        }
    }
}

#[derive(Default)]
struct DeltaTime(f32);

fn main() {
    use specs::{Builder, World, WorldExt};

    let mut world = World::new();
    world.insert(DeltaTime(0.05));

    world.register::<Position>();
    world.register::<Velocity>();

    world
        .create_entity()
        .with(Position { x: 4., y: 7. })
        .build();

    world
        .create_entity()
        .with(Position { x: 2., y: 5. })
        .with(Velocity { x: 1., y: 2. })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(MyWorld, "greet_world", &[])
        .with(UpdatePosition, "update_position", &["greet_world"])
        .with(MyWorld, "greet_world_updated", &["update_position"])
        .build();

    dispatcher.dispatch(&mut world);

    world.maintain();
}
