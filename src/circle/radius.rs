use crate::circle::Circle;

impl<T> Circle<T> {
  pub fn get_radius(&self) -> &T {
    &self.r
  }
}
