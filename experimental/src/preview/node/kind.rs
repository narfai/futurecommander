/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::{
    path::{ PathBuf },
    cmp::Ordering
};

use crate::{
    Result, FileSystemError,
    FileType, FileTypeExt
};

use crate::preview::{
    node::PreviewNode
};

//TODO  Rename in FileType and replace the actual filesystem FileTypeExt impl
#[derive(Debug, Clone, Eq)]
pub enum PreviewNodeKind {
    Directory(Vec<PreviewNode>),
    File(Option<PathBuf>),
    Symlink(PathBuf),
    Deleted
}

impl PreviewNodeKind {
    pub fn is_directory(&self) -> bool {
        matches!(self, PreviewNodeKind::Directory(_))
    }

    pub fn is_file(&self) -> bool { matches!(self, PreviewNodeKind::File(_)) }

    pub fn is_symlink(&self) -> bool { matches!(self, PreviewNodeKind::Symlink(_)) }

    pub fn is_deleted(&self) -> bool {
        matches!(self, PreviewNodeKind::Deleted)
    }
}

impl Ord for PreviewNodeKind {
    fn cmp(&self, other: &Self) -> Ordering {
        use PreviewNodeKind::*;
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

impl PartialOrd for PreviewNodeKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PreviewNodeKind {
    fn eq(&self, other: &Self) -> bool {
        use PreviewNodeKind::*;
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

impl FileTypeExt for &PreviewNodeKind {
    fn into_virtual_file_type(self) -> Result<FileType> {
        match self {
            PreviewNodeKind::Symlink(_) => Ok(FileType::Symlink),
            PreviewNodeKind::Directory(_) => Ok(FileType::Directory),
            PreviewNodeKind::File(_) => Ok(FileType::File),
            PreviewNodeKind::Deleted => Err(FileSystemError::Custom(String::from("Delete file has no type")))
        }
    }
}