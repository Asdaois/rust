fn greet(name: &str) {
  println!("Hello, {name}!");
}

fn main() {
  // Immutable variable
  let name = "World";
  greet(name);

  let mut name = "Jose";
  greet(name);
  name = "Alberto";
  greet(name);

  // This variable should have the type annotated
  let number: u32 = "10".parse().unwrap();
  println!("Number is {number}");

  // let number: u32 = "-10".parse().expect("This number don't work because is negative ");
  // println!("Number is {number}");

  let number: i32 = "-10".parse().unwrap();
  println!("Number is {number}");

  // let mut x: u8 = 255;
  // x += 1; ! Rust prevent overflow

  let _x = 5.0;
  let _x: f32 = 5.0;

  let _is_true = true;
  let _is_false: bool = false;

  let char1 = 'a';
  let char2 = '5';
  let char3 = '\u{263a}'; // unicode smiling face

  println!("c1 = {char1}, c2 = {char2}, c3 = {char3}");

  let tuple: (i32, char, f64) = (10, 'a', 10.5);
  println!("first element of tuple {}", tuple.0);

  let array = [10, 20, 30];
  let num1 = array[0];
  println!("number one in array is = {}", num1);

  let mut array = [-10, 20, 30];
  array[0] = 10;
  println!("number one in array is = {}", num1);

  // let arr: [i32; 10] = todo!();
  // println!("arr[0] = {}", arr[0]); //! Panic not yet implemented

  let arr = [0; 10];
  println!("arr[0] = {}", arr[0]);

  let len = arr.len();
  // Panic index out of bound
  // println!("last element is = {}", arr[len]);
  println!("last element is = {}", arr[len - 1]);
}
