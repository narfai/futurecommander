use std::{
    path::{ Path, PathBuf },
    cmp::Ordering
};

use crate::{
    node::{ Node, Kind, Source }
};

#[derive(Debug)]
pub struct NodeItem<'a>{
    parent_path: PathBuf,
    child: &'a Node
}

impl <'a>NodeItem<'a> {
    pub fn parent_path(&self) -> &Path {
        &self.parent_path
    }

    pub fn path(&self) -> PathBuf {
        self.parent_path.join(self.child.name())
    }

    pub fn node(&self) -> &Node {
        self.child
    }

    pub fn source(&self) -> Option<&Path> {
        match self.child.kind() {
            Kind::File(source) => match source {
                Source::Copy(source_path) | Source::Move(source_path) => Some(&source_path),
                _ => None
            },
            _ => None
        }
    }

    pub fn is_contained_by(&self, path: &Path) -> bool {
        for ancestor in self.path().ancestors() {
            if path == ancestor {
                return true;
            }
        }
        false
    }

    pub fn is_deleted(&self) -> bool {
        matches!(self.node().kind(), Kind::Deleted)
    }
}

impl <'a>Eq for NodeItem<'a> {}

impl <'a>PartialEq for NodeItem<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.node().eq(other.node())
    }
}

impl <'a>PartialOrd for NodeItem<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <'a>Ord for NodeItem<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.node().cmp(&other.node())
    }
}
