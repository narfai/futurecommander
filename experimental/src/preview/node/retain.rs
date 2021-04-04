/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::path::{Path, PathBuf};

use crate::Result;

use super::{
    Node,
    NodeFileType
};

pub fn retain(parent: &mut Node, parent_path: PathBuf, predicate: &dyn Fn(&Path, &Node) -> bool) -> Result<()> {
    let new_parent_path = parent_path.join(parent.name());
    if let NodeFileType::Directory(children) = &mut parent.kind {
        children.retain(|node| predicate(new_parent_path.as_path(), node));
        for child in children {
            retain(child, new_parent_path.clone(), predicate)?;
        }
    }
    Ok(())
}

impl Node {
    pub fn retain(&mut self, predicate: &dyn Fn(&Path, &Node) -> bool) -> Result<()> {
        retain(self, PathBuf::from(self.name()), predicate)
    }
}


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use std::{ffi::OsStr, path::Component};

    use super::*;

    #[test]
    fn test_retain() {
        let node_c = Node::new_file(OsStr::new("C"), None);
        let node_b = Node::new_directory_with_children(OsStr::new("B"), vec![node_c.clone()]);
        let node_a = Node::new_file(OsStr::new("A"), None);

        let mut node = Node::new_directory_with_children(
            Component::RootDir.as_os_str(),
            vec![
                node_a.clone(),
                node_b.clone()
            ]
        );

        assert_eq!(&node_a, node.find_at_path(&Path::new("/A")).unwrap());
        assert_eq!(&node_b, node.find_at_path(&Path::new("/B")).unwrap());
        assert_eq!(&node_c, node.find_at_path(&Path::new("/B/C")).unwrap());

        node.retain(&|parent_path, child| parent_path.join(child.name()) != Path::new("/B")).unwrap();

        assert_eq!(&node_a, node.find_at_path(&Path::new("/A")).unwrap());
        assert_eq!(None, node.find_at_path(&Path::new("/B")));
        assert_eq!(None, node.find_at_path(&Path::new("/B/C")));
    }

    //TODO test proper parent_path
}