use std::{
    fmt,
    error,
    io
};

#[derive(Debug)]
pub enum NodeError {
    IoError(io::Error),
    Custom(String),
}

impl From<io::Error> for NodeError {
    fn from(error: io::Error) -> Self {
        NodeError::IoError(error)
    }
}


impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeError::IoError(error) => write!(f, "I/O error {}", error),
            NodeError::Custom(s) => write!(f, "Custom error {}", s),
        }
    }
}

impl error::Error for NodeError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            NodeError::IoError(err) => Some(err),
            _ => None
        }
    }
}
