fn greet(name: &str) {
  println!("Hello, {name}!");
}

// function_name(<argument values: type>)
fn sum(x: i32, y: i32) -> i32 {
  println!("Calculating sum..."); // Statement
  x + y // Expression, return a value
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

  println!("The sum of 7 and 5 is: {}", sum(7, 5));

  let x = 6;
  if x == 5 {
    println!("X is 5");
  } else if x == 9 {
    println!("X is 9");
  } else {
    println!("X is neither 5 or 9")
  }

  let mut x = 0;
  loop {
    println!("X -> {x}");
    x += 1;

    if x == 3 {
      break;
    }
  }

  // Return value with loop
  let mut x = 1;
  let mut i = 0;
  let result = loop {
    println!("Get ${x} or double and give it to the next person");
    x += x;

    i += 1;
    if i > 5 {
      break x;
    }
  };

  println!("Bankrupt I need to give ${result}");

  let array = [10, 20, 30, 40, -1];
  for number in array {
    println!("val = {number}");
  }
}
