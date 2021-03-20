mod error;
mod path;

mod filesystem;
mod preview;
mod container;

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
    filesystem::{ ReadFileSystem, WriteFileSystem, FileType, Metadata, ReadDir }
};

type Result<T> = result::Result<T, FileSystemError>;


#[derive(Debug, Clone)]
pub enum Operation {
    Copy(PathBuf, PathBuf),
    Rename(PathBuf, PathBuf),
    RemoveFile(PathBuf),
    RemoveDir(PathBuf),
    RemoveDirAll(PathBuf),
    CreateDirAll(PathBuf),
    CreateDir(PathBuf),
    CreateFile(PathBuf)
}

impl Operation {
    pub fn apply(&self) -> Result<()> {
        use Operation::*;
        match self {
            Copy(from, to) => { copy(&from, &to)?; },
            Rename(from, to) => { rename(&from, &to)?; },
            RemoveFile(path) => { remove_file(&path)?; },
            RemoveDir(path) => { remove_dir(&path)?; },
            RemoveDirAll(path) => { remove_dir_all(&path)?; },
            CreateDir(path) => { create_dir(&path)?; },
            CreateDirAll(path) => { create_dir_all(&path)?; },
            CreateFile(path) => { File::create(&path)?; }
        };
        Ok(())
    }
}

fn main(){

}