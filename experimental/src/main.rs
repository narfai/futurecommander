mod error;
mod path;

mod filesystem;
mod preview;
mod container;

#[cfg(not(tarpaulin_include))]
pub mod sample;

use std::{
    path::{ PathBuf, Path },
    result,
    fs::{ create_dir, create_dir_all, copy, rename, remove_dir, remove_file, remove_dir_all, File }
};

use self::{
    error::FileSystemError,
    preview::Preview,
};

pub use {
    container::Container,
    filesystem::{ ReadFileSystem, WriteFileSystem, FileType, FileTypeExt, Metadata, MetadataExt, ReadDir }
};

type Result<T> = result::Result<T, FileSystemError>;


fn main(){

}