fn main() {
    let x: u64 = 4_569_967_296;
    let y = u32::max_value();
    let z: u64 = y.into();
    let w: u32 = z.try_into().expect("Conversion error");

    println!("{x}, {y}, {z}, {w}");
    if x == y as u64 {
        println!("x equals y.");
    } else {
        println!("x does not equal y.");
    }
}
