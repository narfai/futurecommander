/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

mod dir_entry;
mod readdir;
mod metadata;
mod file_type;
mod path_ext;
mod read_filesystem;
mod write_filesystem;

pub use self::{
    readdir::{ ReadDir },
    dir_entry::{ DirEntry },
    metadata::{ Metadata },
    file_type::{ FileType }
};

use std::path::Path;

use crate::Result;

pub struct FileSystem;

pub trait ReadFileSystem {
    fn metadata<P: AsRef<Path>>(&self, path: P) -> Result<Metadata>;  // Given a path, query the file system to get information about a file, directory, etc.
    fn read_dir<P: AsRef<Path>>(&self, path: P) -> Result<ReadDir>;   // Returns an iterator over the entries within a directory.
}

pub trait WriteFileSystem : ReadFileSystem {
    fn create_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;      // Creates a new, empty file at the provided path
    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;       // Creates a new, empty directory at the provided path
    fn create_dir_all<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;   // Recursively create a directory and all of its parent components if they are missing.
    fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> Result<u64>;    // Copies the contents of one file to another. This function will also copy the permission bits of the original file to the destination file.
    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> Result<()>;   // Rename a file or directory to a new name, replacing the original file if to already exists.
    fn remove_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;       // Removes an empty directory.
    fn remove_dir_all<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;   // Removes a directory at this path, after removing all its contents. Use carefully!
    fn remove_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()>;      // Removes a file from the filesystem.
}

pub trait FileTypeExt {
    fn into_virtual_file_type(self) -> Result<FileType>;
}

pub trait MetadataExt {
    fn into_virtual_metadata(self) -> Result<Metadata>;
}

pub trait PathExt {
    fn preview_exists<R: ReadFileSystem>(&self, fs: &R) -> bool;
    fn preview_metadata<R: ReadFileSystem>(&self, fs: &R) -> Result<Metadata>;
    fn preview_read_dir<R: ReadFileSystem>(&self, fs: &R) -> Result<ReadDir>;
    fn preview_is_a_file<R: ReadFileSystem>(&self, fs: &R) -> bool;
    fn preview_is_a_dir<R: ReadFileSystem>(&self, fs: &R) -> bool;
}
