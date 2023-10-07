use std::rc::Rc;

pub enum RERC {
  A(i32),
  B(i32, Rc<RERC>),
}
