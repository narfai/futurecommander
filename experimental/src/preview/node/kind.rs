/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::{
    cmp::Ordering,
    path::PathBuf
};

use crate::{
    FileSystemError, FileType,
    FileTypeExt, Result
};
use crate::preview::{
    node::Node
};

//TODO  Rename in FileType and replace the actual filesystem FileTypeExt impl
#[derive(Debug, Clone, Eq)]
pub enum NodeFileType {
    Directory(Vec<Node>),
    File(Option<PathBuf>),
    Symlink(PathBuf),
    Deleted
}

impl NodeFileType {
    pub fn is_directory(&self) -> bool {
        matches!(self, NodeFileType::Directory(_))
    }

    pub fn is_file(&self) -> bool { matches!(self, NodeFileType::File(_)) }

    pub fn is_symlink(&self) -> bool { matches!(self, NodeFileType::Symlink(_)) }

    pub fn is_deleted(&self) -> bool {
        matches!(self, NodeFileType::Deleted)
    }
}

impl Ord for NodeFileType {
    fn cmp(&self, other: &Self) -> Ordering {
        use NodeFileType::*;
        match self {
            Directory(_) => match other {
                Directory(_) => Ordering::Equal,
                File(_) => Ordering::Greater,
                Symlink(_) => Ordering::Greater,
                Deleted => Ordering::Less
            },
            File(_) => match other {
                Directory(_) => Ordering::Less,
                File(_) => Ordering::Equal,
                Symlink(_) => Ordering::Equal,
                Deleted => Ordering::Less
            },
            Symlink(_) => match other {
                Directory(_) => Ordering::Less,
                File(_) => Ordering::Equal,
                Symlink(_) => Ordering::Equal,
                Deleted => Ordering::Less
            },
            Deleted => match other {
                Directory(_) => Ordering::Greater,
                File(_) => Ordering::Greater,
                Symlink(_) => Ordering::Greater,
                Deleted => Ordering::Equal
            }
        }
    }
}

impl PartialOrd for NodeFileType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for NodeFileType {
    fn eq(&self, other: &Self) -> bool {
        use NodeFileType::*;
        match self {
            Directory(left) => match other {
                Directory(right) => left == right,
                _ => false
            },
            File(left) => match other {
                File(right) => left == right,
                _ => false
            },
            Symlink(left) => match other {
                Symlink(right) => left == right,
                _ => false
            },
            Deleted => matches!(other, Deleted)
        }
    }
}

impl FileTypeExt for &NodeFileType {
    fn into_virtual_file_type(self) -> Result<FileType> {
        match self {
            NodeFileType::Symlink(_) => Ok(FileType::Symlink),
            NodeFileType::Directory(_) => Ok(FileType::Directory),
            NodeFileType::File(_) => Ok(FileType::File),
            NodeFileType::Deleted => Err(FileSystemError::ConvertDeletedNodeToFileType)
        }
    }
}