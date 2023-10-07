/*
use generics and trait bounds to achieve polymorphism.
It is extremely performant (zero-cost abstraction) but leads to larger binary
size due to monomorphization.
*/
use crate::oop::geometry::traits::Shape;

pub fn print_area_static_dispatch<T: Shape>(t: &T) {
  t.print_area();
}

/*
use trait objects to determine which type satisfies the
polymorphic interface during runtime. Since there is no monomorphization, it
results in a smaller binary size but has a performance penalty due to lookups
at the runtime. Trait objects cannot be used with generics, so this approach
does not allow the use of generics.
*/
pub fn print_area_dynamic_dispatch(t: &dyn Shape) {
  t.print_area();
}
