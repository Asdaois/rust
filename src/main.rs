// The next line stole the reference
// fn print_string_data(s: String) {
// the function need to borrow it
fn print_string_data(s: &String) {
  println!(
    "string = {s} \nptr = {:?} \nlength = {}, \ncapacity = {}\n",
    s.as_ptr(),
    s.len(),
    s.capacity()
  );
}

fn mutate_string_data(s: &mut String, added_text: &str) {
  s.push_str(added_text);
}
fn main() {
  // Scope Begins
  let mut s = String::from("rust"); // S come into the scope, the
  s.push_str(" program");

  print_string_data(&s); // This function stole the direction of s, dropping the value in the next

  let mut complete_string = s; // value of s moved to complete_string

  // data races
  // let ref_to_cs = &complete_string;

  print_string_data(&complete_string);

  mutate_string_data(&mut complete_string, " for practice mutation in borrow");
  print_string_data(&complete_string);

  // the reference is burrowed so, the code don't compile
  // print_string_data(ref_to_cs);

  let s = String::from("Viva Chavez la lucha sigue ('sarcasmo')");
  let w1 = &s[..4];
  let w2 = &s[11..20];
  println!("Original: {s}");
  println!("based: {}", w1.to_owned() + w2)
} // Scope ends. Value of si s dropped here
  // When the data has a fixed size is stored in STACK otherwise is in the HEAP
