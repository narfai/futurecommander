use std::{ path:: Path };
use crate::{
    Result,
    ReadFileSystem,
    filesystem:: { Metadata, ReadDir }
};

use super::PathExt;

impl PathExt for Path {
    fn preview_exists<R: ReadFileSystem>(&self, fs: &R) -> bool {
        fs.metadata(self).is_ok()
    }

    fn preview_metadata<R: ReadFileSystem>(&self, fs: &R) -> Result<Metadata> {
        fs.metadata(self)
    }

    fn preview_read_dir<R: ReadFileSystem>(&self, fs: &R) -> Result<ReadDir> {
        fs.read_dir(self)
    }

    fn preview_is_a_file<R: ReadFileSystem>(&self, fs: &R) -> bool {
        fs.metadata(self).map(|m| m.is_file()).unwrap_or(false)
    }

    fn preview_is_a_dir<R: ReadFileSystem>(&self, fs: &R) -> bool {
        fs.metadata(self).map(|m| m.is_dir()).unwrap_or(false)
    }
}