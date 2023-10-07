use crate::oop::geometry::traits::Shape;

pub struct Rectangle {
  pub(crate) l: f64,
  pub(crate) w: f64,
}

impl Shape for Rectangle {
  fn area(&self) -> f64 {
    self.l * self.w
  }
}
