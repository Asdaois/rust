use crate::geometric_forms::circle::Circle;

impl<T> Circle<T> {
  pub fn get_radius(&self) -> &T {
    &self.r
  }
}
