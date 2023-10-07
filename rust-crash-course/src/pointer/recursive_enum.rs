pub enum RecursiveEnum {
  B(i32),
  C(i32, Box<RecursiveEnum>),
}
