use std::f32::consts::PI;

use crate::geometric_forms::area_rect::area_rect;
use crate::geometric_forms::circle::Circle;
use crate::geometric_forms::circle_closure::CircleClosure;
use crate::misc::counter_finite::CounterFinite;
use crate::sum::sum_float::sum_float;
use crate::sum::sum_int::sum_int;

mod geometric_forms;
mod misc;
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

  // circle.print_shape();
  // println!("area of {}", circle.area());
  // println!("perimeter {}", circle.perimeter());

  let area_circle: fn(f32) -> f32 = |r| PI * r * r;

  let c = CircleClosure {
    radius: 5.0,
    area: area_circle,
  };

  println!("Area of circle = {:?}", (c.area)(c.radius));

  let num = 5;
  let add_num = |x| x + num;

  let a = add_num(10);
  println!("a = {a}");

  let cf = CounterFinite { count: 0 };

  for i in cf {
    println!("{i}");
  }
}
