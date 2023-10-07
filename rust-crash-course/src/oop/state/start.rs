use super::traits::State;

pub struct Start;

impl State for Start {
  fn process(&self) {
    println!("Started..")
  }
}
