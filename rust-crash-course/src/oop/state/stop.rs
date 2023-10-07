use super::traits::State;

pub struct Stop;

impl State for Stop {
  fn process(&self) {
    println!("Stopped..")
  }
}
