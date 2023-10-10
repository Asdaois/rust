struct Health(i32);
struct Name(&'static str);

trait ComponentVec {
    fn push_none(&mut self);
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: 'static> ComponentVec for Vec<Option<T>> {
    fn push_none(&mut self) {
        self.push(None)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
}

#[derive(Default)]
struct World {
    entities_count: usize,
    components: Vec<Box<dyn ComponentVec>>,
}

impl World {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;

        for component_vec in self.components.iter_mut() {
            component_vec.push_none();
        }

        self.entities_count += 1;
        entity_id
    }

    fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity_id: usize,
        component: ComponentType,
    ) {
        for component_vec in self.components.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>()
            {
                component_vec[entity_id] = Some(component);
                return;
            }
        }

        // No matching component storage exists yet
        let mut new_component_vec: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities_count);

        // All existing entities don't have this component, so we give them `None`
        for _ in 0..self.entities_count {
            new_component_vec.push_none();
        }

        new_component_vec[entity_id] = Some(component);
        self.components.push(Box::new(new_component_vec));
    }

    fn borrow_component_vec<ComponentType: 'static>(&self) -> Option<&Vec<Option<ComponentType>>> {
        for component_vec in self.components.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<Vec<Option<ComponentType>>>()
            {
                return Some(component_vec);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{Health, Name, World};

    #[test]
    fn works() {
        let mut world = World::new();
        // Icarus's health is *not* looking good.
        let id = world.new_entity();
        world.add_component_to_entity(id, Health(100));
        world.add_component_to_entity(id, Name("Prometheus"));

        let id = world.new_entity();
        world.add_component_to_entity(id, Health(-100));
        world.add_component_to_entity(id, Name("Jose"));

        let data = world
            .borrow_component_vec::<Health>()
            .unwrap()
            .iter()
            .zip(world.borrow_component_vec::<Name>().unwrap().iter());

        for (health, name) in
            data.filter_map(|(health, name)| Some((health.as_ref()?, name.as_ref()?)))
        {
            if health.0 < 0 {
                println!("{} has perished!", name.0)
            } else {
                println!("{} is still healthy", name.0)
            }
        }
    }
}
