use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum RERCELL {
  A(i32),
  B(Rc<RefCell<i32>>, Rc<RERCELL>),
}
