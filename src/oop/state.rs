use crate::oop::state::run::Run;
use crate::oop::state::start::Start;
use crate::oop::state::state_context::StateContext;
use crate::oop::state::stop::Stop;

mod run;
mod start;
mod state_context;
mod stop;
mod traits;

pub fn run() {
  let mut state_machine = StateContext::new();
  state_machine.process();
  state_machine.set_state(Box::new(Run {}));
  state_machine.process();
  state_machine.process();
  state_machine.process();
  state_machine.set_state(Box::new(Stop {}));
  state_machine.process();
  state_machine.process();
  state_machine.set_state(Box::new(Start {}));
  state_machine.process();
  state_machine.set_state(Box::new(Run {}));
  state_machine.process();
  state_machine.process();
  println!("Continue running until the infinity");
}
