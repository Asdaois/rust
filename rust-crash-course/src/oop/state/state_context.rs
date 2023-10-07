use super::start::Start;
use super::traits::State;

pub struct StateContext {
  state: Box<dyn State>,
}

impl StateContext {
  pub fn new() -> StateContext {
    StateContext {
      state: Box::new(Start {}),
    }
  }

  pub fn set_state(&mut self, s: Box<dyn State>) {
    self.state = s;
  }
  pub fn process(&self) {
    self.state.process();
  }
}
