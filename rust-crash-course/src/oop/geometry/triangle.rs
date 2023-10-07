use crate::oop::geometry::traits::Shape;

pub struct Triangle {
  pub(crate) a: f64,
  pub(crate) b: f64,
  pub(crate) c: f64,
}

impl Shape for Triangle {
  fn area(&self) -> f64 {
    let s = (self.a + self.b + self.c) / 2.0;
    (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
  }
}
