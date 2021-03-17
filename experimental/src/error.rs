use std::{
    fmt,
    error,
    io
};

#[derive(Debug)]
pub enum FileSystemError {
    IoError(io::Error),
    Custom(String),
}

impl From<io::Error> for FileSystemError {
    fn from(error: io::Error) -> Self {
        FileSystemError::IoError(error)
    }
}


impl fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileSystemError::IoError(error) => write!(f, "I/O error {}", error),
            FileSystemError::Custom(s) => write!(f, "Custom error {}", s),
        }
    }
}

impl error::Error for FileSystemError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            FileSystemError::IoError(err) => Some(err),
            _ => None
        }
    }
}
