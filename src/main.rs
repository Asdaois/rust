use std::cell::RefCell;
use std::rc::Rc;

use crate::pointer::recursive_enum::RecursiveEnum;
use crate::pointer::recursive_enum_ref_cell::RERCELL;
use crate::pointer::recursive_enum_reference_counting::RERC;

mod pointer;

fn main() {
  let x = 10;
  let y = 20;
  let x_p = &x;

  println!("x = {x}, &x = {:p}", &x);
  println!("y = {y}, &y = {:p}", &y);
  println!("x_p = {:p}", x_p);

  let x = Box::new(10.5);
  println!("x = {x}");

  let x = RecursiveEnum::C(
    2,
    Box::new(RecursiveEnum::C(1, Box::new(RecursiveEnum::B(2)))),
  );

  let x_size = std::mem::size_of_val(&x);
  println!("size of x = {x_size}");

  let x = Rc::new(RERC::B(1, Rc::new(RERC::B(2, Rc::new(RERC::A(3))))));
  println!("count of x = {}", Rc::strong_count(&x));
  // add y
  let _y = RERC::B(5, Rc::clone(&x));
  println!("count of x = {}", Rc::strong_count(&x));
  {
    let _z = RERC::B(10, Rc::clone(&x));
    println!("count of x = {}", Rc::strong_count(&x));
  }

  println!("count of x = {}", Rc::strong_count(&x));

  let x = Rc::new(RefCell::new(10));
  let y = Rc::new(RERCELL::B(Rc::clone(&x), Rc::new(RERCELL::A(1))));
  println!("y = {:?}", y);
  *x.borrow_mut() += 100;
  println!("y mutated = {:?}", y);
}
