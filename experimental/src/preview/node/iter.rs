/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::{
    iter,
    path::PathBuf
};

use super::Node;

fn iter<'a>(node: &'a Node, parent_path: PathBuf) -> Box<dyn Iterator<Item = (PathBuf, &Node)> + 'a>{
    if let Some(children) = node.children() {
        let new_parent_path = parent_path.join(node.name());
        Box::new(
            iter::once((parent_path, node))
                .chain(
                    children.iter()
                        .map(move |n| iter(n,new_parent_path.clone()))
                        .flatten()
                )
        )
    } else {
        Box::new(iter::once((parent_path, node)))
    }
}

impl Node {
    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (PathBuf, &Node)> + 'a> {
        iter(self, PathBuf::from(self.name()))
    }
}


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use std::{
        ffi::OsStr,
        path::Component
    };

    use super::*;

    #[test]
    fn iter_recursively() {
        let node_c = Node::new_file(OsStr::new("C"), None);
        let node_b = Node::new_directory_with_children(OsStr::new("B"), vec![node_c.clone()]);
        let node_a = Node::new_file(OsStr::new("A"), None);

        let node = Node::new_directory_with_children(Component::RootDir.as_os_str(),
                                                     vec![
                node_a.clone(),
                node_b.clone()
            ]
        );

        let collection : Vec<PathBuf>= node.iter().map(|(path, _item)| path.to_path_buf()).collect();
        collection.contains(&PathBuf::from("/"));
        collection.contains(&PathBuf::from("/A"));
        collection.contains(&PathBuf::from("/B"));
        collection.contains(&PathBuf::from("/B/C"));
    }
}