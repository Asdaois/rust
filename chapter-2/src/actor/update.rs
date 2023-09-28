use super::Actor;

impl Actor {
    /// Update functions called from Game
    pub fn update(&self, delta_time: f64) {}

    /// Update all the components attached to the actor
    fn update_components(delta_time: f64) {}
}
