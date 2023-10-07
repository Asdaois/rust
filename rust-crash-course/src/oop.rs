use crate::oop::custom_stack::CustomStack;

mod custom_stack;
mod geometry;
mod state;

pub fn run() {
  let mut custom_stack = CustomStack::new();

  custom_stack.push(5);
  custom_stack.push(15);
  custom_stack.push(2);

  println!("Max value in stack = {}", custom_stack.get_max());
  custom_stack.pop();
  custom_stack.pop();

  println!("Max value in stack = {}", custom_stack.get_max());

  geometry::run();

  state::run();
}
