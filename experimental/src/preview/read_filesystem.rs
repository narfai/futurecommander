use std::{
    path::{ Path },
};

use super::{
    Preview,
    kind::Kind
};

use crate::{
    Result,
    FileSystemError,
    ReadFileSystem,
    filesystem::{
        Metadata,
        ReadDir,
        MetadataExt,
    }
};

impl ReadFileSystem for Preview {
    /// Errors :
    /// * The user lacks permissions to perform `metadata` call on `path`.
    /// * `path` does not exist.
    fn metadata<P: AsRef<Path>>(&self, path: P) -> Result<Metadata> {
        let path = path.as_ref();
        if let Some(node) = self.root.find_at_path(path)? {
            if node.is_deleted(){
                Err(FileSystemError::Custom(String::from("Path does not exists")))
            } else {
                node.into_virtual_metadata()
            }
        } else if path.exists() {
            path.metadata()?.into_virtual_metadata()
        } else {
            Err(FileSystemError::Custom(String::from("Path does not exists")))
        }
    }

    /// Errors :
    /// * The provided `path` doesn't exist.
    /// * The process lacks permissions to view the contents.
    /// * The `path` points at a non-directory file.
    fn read_dir<P: AsRef<Path>>(&self, path: P) -> Result<ReadDir> {
        let path = path.as_ref();
        if let Some(node) = self.root.find_at_path(path)? {
            if node.is_deleted(){
                Err(FileSystemError::Custom(String::from("Path does not exists")))
            } else if let Kind::Directory(children) = node.kind() {
                Ok(ReadDir::new(path, children.iter().cloned().collect()))
            } else {
                Err(FileSystemError::Custom(String::from("Not a directory")))
            }
        } else if path.exists() {
            if path.is_dir() {
                Ok(ReadDir::new(path, Vec::new()))
            } else {
                Err(FileSystemError::Custom(String::from("Not a directory")))
            }
        } else {
            Err(FileSystemError::Custom(String::from("Path does not exists")))
        }
    }
}