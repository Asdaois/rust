mod impl_shape_utils;
mod radius;

#[derive(Debug)]
pub struct Circle<T> {
  pub cx: T,
  pub cy: T,
  pub r: T,
}
