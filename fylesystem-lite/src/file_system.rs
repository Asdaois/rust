use crate::one_in::one_in;

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

pub fn open(file: File) -> Result<File, String> {
    if one_in(10000) {
        return Err("Interrupted by signal".to_string());
    }

    Ok(file)
}

pub fn close(file: File) -> Result<File, String> {
    if one_in(10000) {
        return Err("Interrupted by signal".to_string());
    }

    Ok(file)
}
