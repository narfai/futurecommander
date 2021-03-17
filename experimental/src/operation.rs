use std::{
    path::{ PathBuf }
};

#[derive(Debug, Clone)]
pub enum Operation {
    Copy(PathBuf, PathBuf),
    Rename(PathBuf, PathBuf),
    RemoveFile(PathBuf),
    RemoveDir(PathBuf),
    RemoveDirAll(PathBuf),
    CreateDirAll(PathBuf),
    CreateFile(PathBuf)
}