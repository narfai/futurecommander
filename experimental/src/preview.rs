
use std::{
    path::Path
};

use self::super::{
    FileSystemError,
    WriteFileSystem,
    ReadFileSystem
};

use crate::filesystem::{
    Metadata,
    ReadDir,
    DirEntry
};

pub struct Preview {

}

impl ReadFileSystem for Preview {
    /// Errors :
    /// * The user lacks permissions to perform `metadata` call on `path`.
    /// * `path` does not exist.
    fn metadata<P: AsRef<Path>>(&self, path: P) -> Result<Metadata, FileSystemError> {
        unimplemented!()
    }

    /// Errors :
    /// * The provided `path` doesn't exist.
    /// * The process lacks permissions to view the contents.
    /// * The `path` points at a non-directory file.
    fn read_dir<P: AsRef<Path>>(&self, path: P) -> Result<ReadDir, FileSystemError> {
        unimplemented!()
    }
}

impl WriteFileSystem for Preview {
    /**
     * Errors :
     * - User lacks permissions to create directory at `path`.
     * - A parent of the given path doesn't exist.
     * - `path` already exists.
     */
    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<(), FileSystemError> {

        Ok(())
    }

    fn create_dir_all<P: AsRef<Path>>(&mut self, path: P) -> Result<(), FileSystemError> {
        Ok(())
    }

    /**
     * Errors :
     * - The `from` path is not a file.
     * - The `from` file does not exist.
     * - The current process does not have the permission rights to access
     * - `from` or write `to`.
     */
    fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> Result<u64, FileSystemError> {
        Ok(0)
    }

    /// Because of this, the behavior when both `from` and `to` exist differs. On
    /// Unix, if `from` is a directory, `to` must also be an (empty) directory. If
    /// `from` is not a directory, `to` must also be not a directory. In contrast,
    /// on Windows, `from` can be anything, but `to` must *not* be a directory.
    /// Errors :
    /// * `from` does not exist.
    /// * The user lacks permissions to view contents.
    /// * `from` and `to` are on separate filesystems.
    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> Result<(), FileSystemError> {
        Ok(())
    }

    /// Errors :
    /// * `path` doesn't exist.
    /// * `path` isn't a directory.
    /// * The user lacks permissions to remove the directory at the provided `path`.
    /// * The directory isn't empty.
    fn remove_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<(), FileSystemError> {
        Ok(())
    }

    /// Errors:  cf remove_file & remove_dir
    fn remove_dir_all<P: AsRef<Path>>(&mut self, path: P) -> Result<(), FileSystemError> {
        Ok(())
    }

    /// Errors:
    /// * `path` points to a directory.
    /// * The file doesn't exist.
    /// * The user lacks permissions to remove the file.
    fn remove_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), FileSystemError> {
        Ok(())
    }
}
