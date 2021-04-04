/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::result;

pub use {
    container::Container,
    error::FileSystemError,
    filesystem::{
        DirEntry, FileType,
        FileTypeExt, Metadata,
        MetadataExt, PathExt,
        ReadDir, ReadFileSystem,
        WriteFileSystem
    },
    preview::{Preview, PreviewNode}
};

mod error;
mod path;

mod filesystem;
mod preview;
mod container;

#[cfg(not(tarpaulin_include))]
pub mod sample;

type Result<T> = result::Result<T, FileSystemError>;


fn main(){

}