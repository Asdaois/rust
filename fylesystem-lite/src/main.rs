use fyle_system::{close, open, File};

mod fyle_system;

fn main() {
    let mut file = File::from("f1.txt");
    open(&mut file);
    close(&mut file);
\}
