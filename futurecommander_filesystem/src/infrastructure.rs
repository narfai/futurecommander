// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

mod errors;
mod adapter;
mod real;
mod virt;

use std::path::{ Path };
use crate::{
    Entry,
    EntryCollection,
    QueryError
};
pub use self::{
    errors::InfrastructureError,
    adapter::FileSystemAdapter,
    real::filesystem::RealFileSystem,
    virt::{
        filesystem::VirtualFileSystem,
        entry_status::VirtualStatus
    }
};

pub trait ReadableFileSystem {
    type Item : Entry;
    fn read_dir(&self, path: &Path) -> Result<EntryCollection<Self::Item>,QueryError>;
    fn status(&self, path: &Path) -> Result<Self::Item, QueryError>;
    fn read_maintained(&self, path: &Path) -> Result<EntryCollection<Self::Item>,QueryError> {
        self.read_dir(path)
    }
    fn is_directory_empty(&self, path: &Path) -> Result<bool, QueryError>;
}

pub trait WriteableFileSystem: ReadableFileSystem {
    //Write API Interface
    fn create_empty_directory(&mut self, path: &Path) -> Result<(), InfrastructureError>;
    fn create_empty_file(&mut self, path: &Path) -> Result<(), InfrastructureError>;
    fn copy_file_to_file(&mut self, source: &Path, destination: &Path) -> Result<(), InfrastructureError>;
    fn move_file_to_file(&mut self, source: &Path, destination: &Path) -> Result<(), InfrastructureError>;
    fn bind_directory_to_directory(&mut self, source: &Path, destination: &Path) -> Result<(), InfrastructureError>;
    fn remove_file(&mut self, path: &Path) -> Result<(), InfrastructureError>;
    fn remove_empty_directory(&mut self, path: &Path) -> Result<(), InfrastructureError>;
    fn remove_maintained_empty_directory(&mut self, path: &Path) -> Result<(), InfrastructureError> {
        self.remove_empty_directory(path)
    }
}