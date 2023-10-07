pub trait Shape {
  fn area(&self) -> f64;
  fn print_area(&self) {
    println!("Area = {}", &self.area())
  }
}
