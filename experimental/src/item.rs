use std::{
    path::{ Path, PathBuf },
    cmp::Ordering
};

use crate::{
    node::{ Node, Kind }
};

#[derive(Debug, Clone)]
pub struct NodeItem<'a>{
    path: PathBuf,
    child: &'a Node
}

impl <'a>NodeItem<'a> {
    pub fn new(parent_path: PathBuf, child: &'a Node) -> Self {
        NodeItem {
            path: parent_path.join(child.name()),
            child
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn node(&self) -> &Node {
        self.child
    }

    pub fn source(&self) -> Option<&Path> {
        match self.child.kind() {
            Kind::File(Some(source)) => Some(&source),
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
}

impl <'a>Eq for NodeItem<'a> {}

impl <'a>PartialEq for NodeItem<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.node().eq(other.node())
    }
}
