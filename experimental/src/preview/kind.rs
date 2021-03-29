use std::{
    collections::HashSet,
    path::{ PathBuf },
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

impl From<Kind> for Option<FileType> {
    fn from(kind: Kind) -> Option<FileType> {
        match kind {
            Kind::Directory(_) => Some(FileType::Directory),
            Kind::File(_) => Some(FileType::File),
            Kind::Symlink(_) => Some(FileType::Symlink),
            Kind::Deleted => None
        }
    }
}