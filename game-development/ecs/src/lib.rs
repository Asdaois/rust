use std::{
    borrow::BorrowMut,
    cell::{RefCell, RefMut},
};

struct Health(i32);
#[derive(PartialEq, Eq)]
struct Name(&'static str);

trait ComponentVec {
    fn push_none(&mut self);
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

type ComponentVecImpl<T> = RefCell<Vec<Option<T>>>;

impl<T: 'static> ComponentVec for ComponentVecImpl<T> {
    fn push_none(&mut self) {
        self.get_mut().push(None)
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
                .downcast_mut::<ComponentVecImpl<ComponentType>>()
            {
                component_vec.get_mut()[entity_id] = Some(component);
                return;
            }
        }

        // No matching component storage exists yet
        let mut new_component_vec: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities_count);

        // All existing entities don't have this component, so we give them `None`
        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        new_component_vec[entity_id] = Some(component);
        self.components
            .push(Box::new(RefCell::new(new_component_vec)));
    }

    fn borrow_component_vec<ComponentType: 'static>(
        &self,
    ) -> Option<RefMut<Vec<Option<ComponentType>>>> {
        for component_vec in self.components.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<ComponentVecImpl<ComponentType>>()
            {
                return Some(component_vec.borrow_mut());
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
        world.add_component_to_entity(id, Health(100));
        world.add_component_to_entity(id, Name("Jose"));

        let mut healths = world.borrow_component_vec::<Health>().unwrap();
        let mut names = world.borrow_component_vec::<Name>().unwrap();
        let zip = healths.iter_mut().zip(names.iter_mut());
        let iter = zip.filter_map(|(health, name)| Some((health.as_mut()?, name.as_ref()?)));

        for (health, name) in iter {
            println!("name: {:?}", name.0 == "Jose");
            if name.0 == "Jose" {
                *health = Health(-100)
            }
        }

        let zip = healths.iter_mut().zip(names.iter_mut());
        let iter = zip.filter_map(|(health, name)| Some((health.as_mut()?, name.as_ref()?)));
        for (health, name) in iter {
            println!("health: {:?}", health.0);
            if health.0 < 0 {
                println!("{} is dead", name.0);
                continue;
            }

            println!("{} is still healthy", name.0)
        }
    }
}
