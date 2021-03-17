use std::{ path:: Path };
use crate::{
    Result,
    ReadFileSystem,
    filesystem:: { Metadata, ReadDir }
};

pub trait PathExt {
    fn exists_virtually<R: ReadFileSystem>(&self, fs: &R) -> bool;
    fn metadata_virtually<R: ReadFileSystem>(&self, fs: &R) -> Result<Metadata>;
    fn read_dir_virtually<R: ReadFileSystem>(&self, fs: &R) -> Result<ReadDir>;
    fn is_a_file_virtually<R: ReadFileSystem>(&self, fs: &R) -> bool;
    fn is_a_dir_virtually<R: ReadFileSystem>(&self, fs: &R) -> bool;
}

impl PathExt for Path {
    fn exists_virtually<R: ReadFileSystem>(&self, fs: &R) -> bool {
        fs.metadata(self).is_ok()
    }

    fn metadata_virtually<R: ReadFileSystem>(&self, fs: &R) -> Result<Metadata> {
        fs.metadata(self)
    }

    fn read_dir_virtually<R: ReadFileSystem>(&self, fs: &R) -> Result<ReadDir> {
        fs.read_dir(self)
    }

    fn is_a_file_virtually<R: ReadFileSystem>(&self, fs: &R) -> bool {
        fs.metadata(self).map(|m| m.is_file()).unwrap_or(false)
    }

    fn is_a_dir_virtually<R: ReadFileSystem>(&self, fs: &R) -> bool {
        fs.metadata(self).map(|m| m.is_dir()).unwrap_or(false)
    }
}