/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::{
    path::{Component, Path}
};

use crate::{
    FileSystemError, path::normalize,
    Result
};

use super::{
    Node, NodeFileType
};

fn insert_at_path(parent: &mut Node, target_parent_path: &Path, node: Node) -> Result<()> {
    let mut components = target_parent_path.components();
    if let NodeFileType::Directory(children) = &mut parent.kind {
        match components.next() {
            Some(Component::Normal(next)) => {
                if let Some(child) = children.iter_mut().find(|node| node.name() == next) {
                    insert_at_path(child, components.as_path(), node)?;
                } else {
                    let mut child = Node::new_directory(next);
                    insert_at_path(&mut child,components.as_path(), node)?;
                    children.push(child);
                };
                Ok(())
            },
            Some(_) => insert_at_path(parent, components.as_path(), node),
            None => if children.contains(&node) {
                Err(FileSystemError::NodeAlreadyExists(node))
            } else {
                children.push(node);
                Ok(())
            }
        }
    } else {
        Err(FileSystemError::ParentNodeIsNotADirectory(parent.clone()))
    }
}

impl Node {
    pub fn insert_at_path(&mut self, target_parent_path: &Path, node: Node) -> Result<()> {
        insert_at_path(
            self,
            normalize(target_parent_path).as_path(),
            node
        )
    }
}


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use std::{ffi::OsStr};

    use super::*;

    #[test]
    fn test_insertion_at_root() {
        let node_a = Node::new_directory(OsStr::new("A"));
        let node_d = Node::new_directory(OsStr::new("D"));
        let node_h = Node::new_file(OsStr::new("H"), None);
        let mut node = Node::default();

        node.insert_at_path(&Path::new("/"), node_a.clone()).unwrap();
        node.insert_at_path(&Path::new("/A"), node_d.clone()).unwrap();
        node.insert_at_path(&Path::new("/A/D"), node_h.clone()).unwrap();

        assert_eq!(&node_a, node.find_at_path(&Path::new("/A")).unwrap());
        assert_eq!(&node_d, node.find_at_path(&Path::new("/A/D")).unwrap());
        assert_eq!(&node_h, node.find_at_path(&Path::new("/A/D/H")).unwrap());
    }

    #[test]
    fn test_dangling_insertion() {
        let mut node = Node::new_directory(OsStr::new("/"));
        let node_h = Node::new_file(OsStr::new("H"), None);

        node.insert_at_path(&Path::new("/A/D"), node_h.clone()).unwrap();
        assert_eq!(&node_h, node.find_at_path(&Path::new("/A/D/H")).unwrap());
    }
}
