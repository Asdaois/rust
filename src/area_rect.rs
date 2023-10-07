use std::ops::Mul;

pub fn area_rect<T: Mul<Output = T>>(length: T, width: T) -> T {
  length * width
}
