use crate::misc::counter_finite::CounterFinite;

impl Iterator for CounterFinite {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.count < 10 {
      self.count += 1;
      return Some(self.count);
    }

    None
  }
}
