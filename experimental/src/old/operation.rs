use std::{
    path::{ PathBuf }
};

use crate::{
    node::{ Node, Kind },
    error::{ NodeError }
};

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
    pub fn apply(&self, node: Node) -> Result<Node, NodeError> {
        use Operation::*;
        unimplemented!()
    }
}