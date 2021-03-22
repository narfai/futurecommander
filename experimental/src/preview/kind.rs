use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    ffi::{ OsString, OsStr },
    path::{ Path, PathBuf, Components, Component },
    cmp::Ordering,
    iter
};

use crate::{
    FileType
};

use super::{
    node::Node
};

#[derive(Debug, Clone)]
pub enum Kind {
    Directory(HashSet<Node>),
    File(Option<PathBuf>),
    Symlink(PathBuf),
    Deleted
}

impl Into<Option<FileType>> for Kind {
    fn into(self) -> Option<FileType> {
        match self {
            Kind::Directory(_) => Some(FileType::Directory),
            Kind::File(_) => Some(FileType::File),
            Kind::Symlink(_) => Some(FileType::Symlink),
            Deleted => None
        }
    }
}