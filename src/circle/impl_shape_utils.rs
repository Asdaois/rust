use std::f64::consts::PI;

use crate::circle::Circle;
use crate::shape_utils::ShapeUtils;

impl ShapeUtils for Circle<f64> {
  fn print_shape(&self) {
    println!(
      "Circle: [c = ({:?},{:?}), r = {:?}",
      self.cx, self.cy, self.r
    );
  }

  fn area(&self) -> f64 {
    PI * self.r * self.r
  }

  fn perimeter(&self) -> f64 {
    2. * PI * self.r
  }
}
