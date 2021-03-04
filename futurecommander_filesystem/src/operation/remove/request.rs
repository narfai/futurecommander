use std::path::{ PathBuf, Path };

use crate::operation::{ Request };

#[derive(Clone)]
pub struct RemoveRequest {
    path: PathBuf
}

impl RemoveRequest {
    pub fn new(path: PathBuf) -> Self {
        RemoveRequest { path }
    }

    pub fn path(&self) -> &Path { &self.path }
}

impl Request for RemoveRequest {}