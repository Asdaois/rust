//! Simulating files one step at a time

use self::file_state::FileState;

mod file_state;
mod impl_display;
mod read;

/// Represents a "file"
/// which probably lives on a file system
#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
}

impl File {
    /// Creates a new [`File`].
    /// New files ar assumed to be empty, but a name is required
    /// if the data is empty, a vec![] can be passed
    pub fn new(name: &str, data: Vec<u8>) -> File {
        File {
            name: String::from(name),
            data,
            state: FileState::Closed,
        }
    }

    /// Returns the length of this [`File`] in bytes.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the name of this [`File`].
    pub fn name(&self) -> String {
        self.name.clone()
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
