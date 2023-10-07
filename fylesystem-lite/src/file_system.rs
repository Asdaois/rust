mod read;
#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
}

impl File {
    pub fn new(name: &str, data: Vec<u8>) -> File {
        File {
            name: String::from(name),
            data,
        }
    }
}

pub fn open(file: &mut File) -> bool {
    true
}
pub fn close(file: &mut File) -> bool {
    true
}
