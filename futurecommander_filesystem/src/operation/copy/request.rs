use std::path::{ PathBuf, Path };

use crate::operation::{ Request };

#[derive(Clone)]
pub struct CopyRequest {
    source: PathBuf,
    destination: PathBuf
}

impl CopyRequest {
    pub fn new(source: PathBuf, destination: PathBuf) -> Self {
        CopyRequest { source, destination }
    }

    pub fn source(&self) -> &Path { &self.source }

    pub fn destination(&self) -> &Path { &self.destination }
}

impl Request for CopyRequest {}