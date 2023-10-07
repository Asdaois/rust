use self::file_state::FileState;

mod file_state;
mod impl_display;
mod read;

#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
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
