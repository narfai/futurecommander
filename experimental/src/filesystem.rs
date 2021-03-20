mod readdir;
mod metadata;
mod path_ext;

use std::{
    path::{ Path, PathBuf },
    ffi::OsString
};

use crate::{
    Result,
    error::FileSystemError
};

pub use self::{
    readdir::{ ReadDir, DirEntry },
    metadata::{ FileType, Metadata, FileTypeExt, MetadataExt },
    path_ext::{ PathExt }
};


pub trait ReadFileSystem {
    fn metadata<P: AsRef<Path>>(&self, path: P) -> Result<Metadata>;  // Given a path, query the file system to get information about a file, directory, etc.
    fn read_dir<P: AsRef<Path>>(&self, path: P) -> Result<ReadDir>;   // Returns an iterator over the entries within a directory.
}

pub trait WriteFileSystem : ReadFileSystem {
    fn create_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;       // Creates a new, empty directory at the provided path
    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;       // Creates a new, empty directory at the provided path
    fn create_dir_all<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;   // Recursively create a directory and all of its parent components if they are missing.
    fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> Result<u64>;    // Copies the contents of one file to another. This function will also copy the permission bits of the original file to the destination file.
    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> Result<()>;   // Rename a file or directory to a new name, replacing the original file if to already exists.
    fn remove_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;       // Removes an empty directory.
    fn remove_dir_all<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;   // Removes a directory at this path, after removing all its contents. Use carefully!
    fn remove_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;      // Removes a file from the filesystem.
}

