use std::io;
use std::path::{ Path, PathBuf };
use std::{ error, fmt };

#[derive(Debug)]
pub enum VfsError {
    IoError(io::Error),
    HasNoSource(PathBuf),
    IsNotADirectory(PathBuf),
    DoesNotExists(PathBuf),
    VirtuallyDoesNotExists(PathBuf),
    AlreadyExists(PathBuf),
}

impl From<io::Error> for VfsError {
    fn from(error: io::Error) -> Self {
        VfsError::IoError(error)
    }
}

impl fmt::Display for VfsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VfsError::IoError(ref err) => write!(f, "IO error: {}", err),
            VfsError::HasNoSource(err) => write!(f, "Path {} : Has no source defined virtually", err.as_os_str().to_string_lossy()),
            VfsError::IsNotADirectory(err) => write!(f, "Path {} is not a directory", err.as_os_str().to_string_lossy()),
            VfsError::DoesNotExists(err) => write!(f, "Path {} does not exists", err.as_os_str().to_string_lossy()),
            VfsError::VirtuallyDoesNotExists(err) => write!(f, "Path {} : Virtually does not exists", err.as_os_str().to_string_lossy()),
            VfsError::AlreadyExists(err) => write!(f, "Path {} : Already exists", err.as_os_str().to_string_lossy()),
        }
    }
}

impl error::Error for VfsError {
    fn cause(&self) -> Option<&error::Error> {
        match self {
            VfsError::IoError(err) => Some(err),
            _ => None
        }
    }
}
