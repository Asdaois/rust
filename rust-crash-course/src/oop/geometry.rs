use traits::Shape;

use crate::oop::geometry::circle::Circle;
use crate::oop::geometry::print_area::{print_area_dynamic_dispatch, print_area_static_dispatch};
use crate::oop::geometry::rectangle::Rectangle;
use crate::oop::geometry::triangle::Triangle;

pub mod circle;
mod print_area;
pub mod rectangle;
mod traits;
pub mod triangle;

pub(crate) fn run() {
  let r1 = Rectangle { l: 10.0, w: 20.0 };
  print_area_static_dispatch(&r1);
  print_area_dynamic_dispatch(&r1);
  let t1 = Triangle {
    a: 3.0,
    b: 4.0,
    c: 5.0,
  };
  print_area_static_dispatch(&t1);
  print_area_dynamic_dispatch(&t1);

  let c1 = Circle { r: 5.0 };
  print_area_static_dispatch(&c1);
  print_area_dynamic_dispatch(&c1);
}
