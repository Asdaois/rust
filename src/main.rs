use std::collections::HashMap;

#[derive(Debug)]
struct Employee {
  name: String,
  age: u8,
  email_id: String,
  experience: u8,
  location: Location,
}

impl Employee {
  fn get_name(&self) -> &str {
    &self.name
  }

  fn set_age(&mut self, age: u8) {
    self.age = age;
  }

  fn print(&self) {
    println!("{:#?}", self);
  }

  fn new(name: String) -> Employee {
    Employee {
      name,
      email_id: "".to_string(),
      age: 35,
      experience: 5,
      location: Location::US(String::from("Unknown")),
    }
  }
}

struct Color(u8, u8, u8, u8);

#[derive(Debug)]
enum Location {
  Venezuela(String),
  US(String),
  UK(String),
}

fn is_odd(num: i8) -> Option<bool> {
  if num % 2 == 1 {
    return Some(true);
  }

  return None;
}

fn get_is_odd_message(response: Option<bool>) -> &'static str {
  match response {
    Option::Some(true) => "Is odd",
    None => "Is not odd",
    _ => "",
  }
}

fn main() {
  let mut employee = Employee::new(String::from("Jose"));
  employee.set_age(28);

  let employee2 = Employee {
    name: String::from("Mar"),
    email_id: employee.email_id.clone(),
    age: 24,
    experience: employee.experience,
    location: Location::Venezuela(String::from("Puerto Ordaz")),
  };

  employee.print();
  employee2.print();

  let _red = Color(255, 0, 0, 255);

  println!("15 is odd? {:?}", is_odd(15));
  println!("12 is odd? {:?}", is_odd(12));

  let is_odd = get_is_odd_message(is_odd(15));
  println!("15 {is_odd}");

  let mut v = vec![1.5, 2.5, 5.0];
  v.push(24.0);

  println!("{:?}", v);

  v.pop();

  println!("{:?}", v);

  println!("First Element: {:?}", v[0]);
  println!("Second Element: {:?}", v[1]);
  println!("Third Element: {:?}", v.get(2));
  println!("Four Element: {:?}", v.get(4));

  // Internalization
  let s1 = String::from("hello"); // 1 byte for character
  let s2 = String::from("Привет"); // 2 byte for character
  println!(
    "s1 = {s1} -> len of {}\ns2 = {s2} -> len of {}",
    s1.len(),
    s2.len()
  );

  println!("{:?}", &s1[0..1]);
  println!("{:?}", &s2[0..4]);

  // Safe way, convert to char, then search for char
  println!("{:?}", s1.chars());
  println!("{:?}", s2.chars());

  let mut currencies = HashMap::new();
  currencies.insert("Venezuela", "Petros");
  currencies.insert("United States", "USD");
  currencies.insert("United Kingdom", "GBP");
  currencies.insert("Venezuela", "BS"); // Overwrite previous value
  println!("{:?}", currencies);
}
