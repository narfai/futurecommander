use std::path::{ PathBuf, Path };

use crate::operation::{ Request };

#[derive(Clone)]
pub struct MoveRequest {
    source: PathBuf,
    destination: PathBuf
}

impl MoveRequest {
    pub fn new(source: PathBuf, destination: PathBuf) -> Self {
        MoveRequest { source, destination }
    }

    pub fn source(&self) -> &Path { &self.source }

    pub fn destination(&self) -> &Path { &self.destination }
}

impl Request for MoveRequest {}