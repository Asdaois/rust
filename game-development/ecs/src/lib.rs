struct Health(i32);
struct Name(&'static str);

#[derive(Default)]
struct World {
    health_components: Vec<Option<Health>>,
    name_components: Vec<Option<Name>>,
}

impl World {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn new_entity(&mut self, health: Option<Health>, name: Option<Name>) {
        self.health_components.push(health);
        self.name_components.push(name);
    }
}
#[cfg(test)]
mod tests {
    use crate::{Health, Name, World};

    #[test]
    fn works() {
        let mut world = World::new();
        // Icarus's health is *not* looking good.
        world.new_entity(Some(Health(-10)), Some(Name("Icarus")));
        // Prometheus is very healthy.
        world.new_entity(Some(Health(100)), Some(Name("Prometheus")));
        // Note that Zeus does not have a `Health` component.
        world.new_entity(None, Some(Name("Zeus")));

        let zip = world
            .health_components
            .iter()
            .zip(world.name_components.iter());

        let with_health_and_name =
            zip.filter_map(|(health, name)| Some((health.as_ref()?, name.as_ref()?)));

        for (health, name) in with_health_and_name {
            if health.0 < 0 {
                println!("{} has perished!", name.0)
            } else {
                println!("{} is still healthy", name.0)
            }
        }
    }
}
