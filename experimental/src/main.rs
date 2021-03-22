mod error;
mod path;

mod filesystem;
mod preview;
mod container;

#[cfg(not(tarpaulin_include))]
pub mod sample;

use std::result;

use self::{
    error::FileSystemError,
    preview::Preview,
};

pub use {
    container::Container,
    filesystem::{
        ReadFileSystem, WriteFileSystem,
        FileType, FileTypeExt,
        Metadata, MetadataExt,
        ReadDir, DirEntry
    }
};

type Result<T> = result::Result<T, FileSystemError>;


fn main(){

}