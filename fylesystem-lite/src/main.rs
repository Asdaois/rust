use file_system::{close, open, File};

mod file_system;

fn main() {
    let mut file = File::new("file.txt", vec![114, 117, 115, 116, 33]);

    let mut buffer = vec![];

    open(&mut file);
    let file_len = file.read(&mut buffer);
    close(&mut file);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", &file);
    println!("{} is {} byte long", &file.name, file_len);
    println!("{}", text);
}
