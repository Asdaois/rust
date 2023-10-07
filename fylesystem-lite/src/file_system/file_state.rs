use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

impl Display for FileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FileState::Open => write!(f, "Open"),
            FileState::Closed => write!(f, "Closed"),
        }
    }
}
