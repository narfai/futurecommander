mod error;
mod path;

mod filesystem;
mod preview;
mod container;

#[cfg(not(tarpaulin_include))]
pub mod sample;

use std::result;

pub use {
    container::Container,
    filesystem::{
        ReadFileSystem, WriteFileSystem,
        FileType, FileTypeExt,
        Metadata, MetadataExt,
        ReadDir, DirEntry
    },
    error::FileSystemError,
    preview::{ Preview, PreviewNode }
};

type Result<T> = result::Result<T, FileSystemError>;


fn main(){

}