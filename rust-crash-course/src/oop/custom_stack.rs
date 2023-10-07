pub struct CustomStack {
  // fields encapsulated
  elements: Vec<i32>,
  max: i32,
}

impl CustomStack {
  pub fn new() -> CustomStack {
    CustomStack {
      elements: vec![],
      max: 0,
    }
  }
  pub fn push(&mut self, elem: i32) {
    self.elements.push(elem);
    self.calculate_max();
  }
  pub fn pop(&mut self) -> Option<i32> {
    let elem = self.elements.pop();
    match elem {
      None => None,
      Some(x) => {
        self.calculate_max();
        Some(x)
      }
    }
  }
  pub fn get_max(&self) -> i32 {
    self.max
  }
  fn calculate_max(&mut self) {
    let m = self.elements.iter().max();
    match m {
      None => self.max = 0,
      Some(x) => self.max = *x,
    }
  }
}
