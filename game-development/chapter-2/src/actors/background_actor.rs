use crate::{
    components::background_component::BackgroundComponent,
    core::{component_system::ComponentSystem, Actor, GameLoop},
    game::engine::Engine,
    math::vector_2::Vector2,
};

#[derive(Default)]
pub struct BackgroundActor {
    position: Vector2,
    textures: Vec<String>,
    component_system: ComponentSystem,
}

impl BackgroundActor {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl GameLoop for BackgroundActor {
    fn draw(&mut self, engine: &mut Engine) {
        for component in self.component_system.components.iter_mut() {
            component.draw(engine);
        }
    }

    fn update(&mut self, engine: &mut Engine, delta_time: f64) {}
    fn init(&mut self, engine: &mut Engine) {
        let mut component = BackgroundComponent::new(None);
        component.add_texture("assets/Farback01.png".into());
        component.add_texture("assets/Farback02.png".into());

        self.component_system.add(component);

        self.component_system.init(engine);
    }
}

impl Actor for BackgroundActor {
    fn set_position(&mut self, position: crate::math::vector_2::Vector2) {
        self.position = position
    }
}
