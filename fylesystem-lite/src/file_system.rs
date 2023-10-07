use crate::one_in::one_in;

mod read;

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    pub fn new(name: &str, data: Vec<u8>) -> File {
        File {
            name: String::from(name),
            data,
            state: FileState::Closed,
        }
    }
}

pub fn open(mut file: File) -> Result<File, String> {
    file.state = FileState::Open;

    Ok(file)
}

pub fn close(mut file: File) -> Result<File, String> {
    file.state = FileState::Closed;

    Ok(file)
}
