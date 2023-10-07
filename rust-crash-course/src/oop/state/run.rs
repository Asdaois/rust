use super::traits::State;

pub struct Run;

impl State for Run {
  fn process(&self) {
    println!("Running..")
  }
}
