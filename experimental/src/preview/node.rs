/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::{
    ffi::{OsStr, OsString},
    hash::{Hash, Hasher},
    path::{Component, Path, PathBuf}
};

pub use kind::NodeFileType;

use crate::{
    FileType, FileTypeExt,
    Metadata, MetadataExt,
    Result
};

mod kind;
mod iter;
mod find_at_path;
mod insert_at_path;
mod retain;
mod tree;

#[derive(Debug, Clone, PartialOrd, Ord)]
pub struct Node {
    name: OsString,
    kind: NodeFileType,
}

impl Default for Node {
    fn default() -> Self {
        Node::new_directory(Component::RootDir.as_os_str())
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name().eq(other.name())
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
    }
}

impl Node {
    pub fn name(&self) -> &OsStr { &self.name }

    pub fn new_directory(name: &OsStr) -> Self {
        Node {
            kind: NodeFileType::Directory(Vec::new()),
            name: name.to_owned(),
        }
    }

    pub fn new_directory_with_children(name: &OsStr, children: Vec<Node>) -> Self {
        Node {
            kind: NodeFileType::Directory(children.into_iter().collect()),
            name: name.to_owned(),
        }
    }

    pub fn new_file(name: &OsStr, source: Option<PathBuf>) -> Self {
        Node {
            kind: NodeFileType::File(source),
            name: name.to_owned(),
        }
    }

    pub fn new_symlink(name: &OsStr, path: &Path) -> Self {
        Node {
            kind: NodeFileType::Symlink(path.to_path_buf()),
            name: name.to_owned(),
        }
    }

    pub fn new_deleted(name: &OsStr) -> Self {
        Node {
            kind: NodeFileType::Deleted,
            name: name.to_owned(),
        }
    }

    pub fn kind(&self) -> &NodeFileType {
        &self.kind
    }

    pub fn source(&self) -> Option<&Path> {
        match &self.kind {
            NodeFileType::File(source) => source.as_ref().map(|src| src.as_path()),
            _ => None
        }
    }

    pub fn children(&self) -> Option<&Vec<Node>> {
        if let NodeFileType::Directory(children) = &self.kind {
            Some(children)
        } else {
            None
        }
    }

    pub fn is_deleted(&self) -> bool {
        self.kind.is_deleted()
    }
}

impl MetadataExt for &Node {
    fn into_virtual_metadata(self) -> Result<Metadata> {
        Ok(
            Metadata {
                file_type: self.into_virtual_file_type()?
            }
        )
    }
}

impl FileTypeExt for &Node {
    fn into_virtual_file_type(self) -> Result<FileType> {
        self.kind.into_virtual_file_type()
    }
}
