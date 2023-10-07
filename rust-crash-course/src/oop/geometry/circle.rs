use std::f64::consts::PI;

use crate::oop::geometry::traits::Shape;

pub struct Circle {
  pub(crate) r: f64,
}

impl Shape for Circle {
  fn area(&self) -> f64 {
    PI * self.r * self.r
  }
}
