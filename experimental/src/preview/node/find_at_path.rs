/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::{
    path::{ Components, Component, Path },
    ffi::{ OsStr },

};

use crate::{
    Result, FileSystemError,
    path::normalize
};

use super::PreviewNode;

impl PreviewNode {
    pub fn find_at_path(&self, path: &Path) -> Result<Option<&PreviewNode>> {
        let buf = normalize(path);
        let mut components = buf.components();
        if let Some(Component::RootDir) = components.next() {
            Ok(
                components.fold(Some(self), |current_node_opt, item|
                    current_node_opt
                        .and_then(|current_node| current_node.children())
                        .and_then(|children|
                            children.iter().find(|n| n.name() == item.as_os_str())
                        )
                )
            )
        } else {
            Err(FileSystemError::Custom("Given path has to be absolute".into()))
        }
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_at_path() {
        let node_c = PreviewNode::new_file("C", None);
        let node_b = PreviewNode::new_directory_with_children("B", vec![node_c.clone()]);
        let node_a = PreviewNode::new_file("A", None);

        let node = PreviewNode::new_directory_with_children(
            "/",
            vec![
                node_a.clone(),
                node_b.clone()
            ]
        );

        assert_eq!(&node, node.find_at_path(&Path::new("/")).unwrap().unwrap());
        assert_eq!(None, node.find_at_path(&Path::new("/DO/NOT/EXISTS")).unwrap());

        assert_eq!(&node_a, node.find_at_path(&Path::new("/A")).unwrap().unwrap());
        assert_eq!(&node_b, node.find_at_path(&Path::new("/B")).unwrap().unwrap());
        assert_eq!(&node_c, node.find_at_path(&Path::new("/B/C")).unwrap().unwrap());

        assert_eq!(&node_a, node.find_at_path(&Path::new("/./././A")).unwrap().unwrap());
        assert_eq!(&node_a, node.find_at_path(&Path::new("/B/../A")).unwrap().unwrap());
        assert_eq!(&node_a, node.find_at_path(&Path::new("/B/../B/../A")).unwrap().unwrap());
    }
}
