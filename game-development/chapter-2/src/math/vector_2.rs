pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Default for Vector2 {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
