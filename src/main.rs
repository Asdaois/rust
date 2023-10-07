use crate::area_rect::area_rect;
use crate::circle::Circle;
use crate::shape_utils::ShapeUtils;
use crate::sum::sum_float::sum_float;
use crate::sum::sum_int::sum_int;

mod area_rect;
mod circle;
mod shape_utils;
mod sum;

fn main() {
  println!("{}", sum_int(4, 5));
  println!("{}", sum_float(4., 5.));

  let circle = Circle {
    r: 5,
    cx: 10,
    cy: 20,
  };

  println!("{:?}", circle);

  println!("Radius of circle is {}", circle.get_radius());

  // let circle = Circle {
  //   r: 5,
  //   cx: 10,
  //   cy: 20., <- Fail because all the fields have to be the same
  // };
  //
  // println!("{:?}", circle);

  println!("The area of a rect is {}", area_rect(10, 5));
  println!("The area of a rect is {}", area_rect(10.2, 5.33));

  let circle = Circle {
    cx: 42.5,
    cy: 24.3,
    r: 12.,
  };

  circle.print_shape();
  println!("area of {}", circle.area());
  println!("perimeter {}", circle.perimeter());
}
