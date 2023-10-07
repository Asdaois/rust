pub struct CircleClosure<T>
where
  T: Fn(f32) -> f32,
{
  pub radius: f32,
  pub area: T,
}
