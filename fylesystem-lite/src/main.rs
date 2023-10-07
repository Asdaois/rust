use file_system::{close, open, File};

mod file_system;
mod one_in;

fn main() {
    let mut file = File::new("file.txt", vec![114, 117, 115, 116, 33]);

    let mut buffer = vec![];

    file = open(file).unwrap();

    file.read(&mut buffer).unwrap();

    file = close(file).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", file);
    println!("{}", file);
    println!("{} is {} byte long", file.name(), file.len());
    println!("{}", text);
}
