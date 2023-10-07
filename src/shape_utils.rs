pub trait ShapeUtils {
  fn print_shape(&self) {
    println!("A geometric shape")
  }
  fn area(&self) -> f64;
  fn perimeter(&self) -> f64;
}
