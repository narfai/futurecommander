use std::path::{ Path, PathBuf };
use std::fs::{ ReadDir };
use crate::{ VirtualDelta, VirtualChildren, VirtualPath, VirtualKind };
use std::{ error, fmt };
use std::io;

#[derive(Debug)]
pub enum VfsError {
    IoError(io::Error),
    ReadDirNoSource(PathBuf),
    ReadDirIsNotADirectory(PathBuf),
    ReadDirDoesNotExists(PathBuf)
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
            VfsError::ReadDirNoSource(err) => write!(f, "Read dir {} : No source defined", err.as_os_str().to_string_lossy()),
            VfsError::ReadDirIsNotADirectory(err) => write!(f, "Read dir {} : Is not a directory", err.as_os_str().to_string_lossy()),
            VfsError::ReadDirDoesNotExists(err) => write!(f, "Read dir {} : Does not exists", err.as_os_str().to_string_lossy()),
        }
    }
}

impl error::Error for VfsError {
    fn cause(&self) -> Option<&error::Error> {
        match self {
            VfsError::IoError(err) => Some(err),
            VfsError::ReadDirNoSource(_) => None,
            VfsError::ReadDirIsNotADirectory(_) => None,
            VfsError::ReadDirDoesNotExists(_) => None,
        }
    }
}

#[derive(Debug)]
pub struct VirtualFileSystem {
    pub add: VirtualDelta,
    pub sub: VirtualDelta
}

impl VirtualFileSystem {
    pub fn new() -> VirtualFileSystem {
        VirtualFileSystem {
            add: VirtualDelta::new(),
            sub: VirtualDelta::new()
        }
    }

    pub fn read_dir(&self, path: &Path) -> Result<VirtualChildren, VfsError> {
        let virtual_state = self.get_virtual_state();
        match self.exists_virtually(path) {
            true => match virtual_state.is_directory(path) {
                Some(true) => match virtual_state.get(path) {
                    Some(virtual_identity) => match virtual_identity.as_source() {
                        Some(source_path) => match VirtualChildren::from_file_system(source_path) {
                            Ok(virtual_children) => Ok(
                                &(&virtual_children - &self.sub.children(path).unwrap())
                                + &self.add.children(path).unwrap()
                            ),
                            Err(error) => Err(VfsError::from(error))
                        },
                        None => Err(VfsError::ReadDirNoSource(path.to_path_buf()))
                    },
                    None => Err(VfsError::ReadDirDoesNotExists(path.to_path_buf()))
                },
                Some(false) => Err(VfsError::ReadDirIsNotADirectory(path.to_path_buf())),
                None => Err(VfsError::ReadDirDoesNotExists(path.to_path_buf()))
            },
            false => match VirtualChildren::from_file_system(path) {
                Ok(virtual_children) => Ok(virtual_children),
                Err(error) => Err(VfsError::from(error))
            }
        }
    }

    pub fn exists_virtually(&self, path: &Path) -> bool {
        self.get_virtual_state().exists(path)
    }

    pub fn is_directory_virtually(&self, path: &Path) -> Option<bool> {
        self.get_virtual_state().is_directory(path)
    }

    pub fn get_add_state(&self) -> VirtualDelta {
        self.add.clone()
    }

    pub fn get_sub_state(&self) -> VirtualDelta {
        self.sub.clone()
    }

    pub fn get_virtual_state(&self) -> VirtualDelta {
        &self.add - &self.sub
    }
}
