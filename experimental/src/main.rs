/*
mod operation_generator;
 */

mod error;
mod path;

mod node;
mod item;
mod operation;

// ================================================================= //

use std::path::{ PathBuf };

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

fn main() {
    println!("Hello, world!");
}
