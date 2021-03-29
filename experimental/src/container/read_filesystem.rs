use std::path::Path;

use crate::{
    Result,
    ReadFileSystem,
    Metadata,
    ReadDir
};

use super::Container;

impl ReadFileSystem for Container {
    fn metadata<P: AsRef<Path>>(&self, path: P) -> Result<Metadata> {
        self.preview.metadata(path)
    }

    fn read_dir<P: AsRef<Path>>(&self, path: P) -> Result<ReadDir> {
        self.preview.read_dir(path)
    }
}