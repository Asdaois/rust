use super::Component;

struct SpriteComponent {
    draw_order: i32,
    texture_height: i32,
    texture_width: i32,
}

impl Component for SpriteComponent {
    fn new(owner: Box<crate::actor::Actor>, draw_order: u32) -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    fn update(&self, delta_time: f64) {
        todo!()
    }

    fn update_order(&self) -> u32 {
        todo!()
    }
}
