use std::ops::Mul;

pub fn area_rect<T: Mul<Output = T>>(length: T, width: T) -> T {
  length * width
}

#[cfg(test)]
mod test {
  use crate::geometric_forms::area_rect::area_rect;

  #[test]
  fn test_area_rect_int() {
    assert_eq!(area_rect(4, 5), 20);
  }

  #[test]
  fn test_area_rect_float() {
    assert_eq!(area_rect(4., 5.), 20.);
  }
}
