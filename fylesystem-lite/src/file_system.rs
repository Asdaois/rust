#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
}

impl File {
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    pub fn with_data(&self, data: Vec<u8>) -> File {
        File {
            name: String::from(&self.name),
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

pub fn read(file: &mut File, save_to: &mut Vec<u8>) -> usize {
    let mut tmp = file.data.clone();
    let read_length = tmp.len();

    save_to.reserve(read_length);
    save_to.append(&mut tmp);

    read_length
}
