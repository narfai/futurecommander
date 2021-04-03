/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::path::{ Component, Path };

use crate::path::normalize;

use super::PreviewNode;

impl PreviewNode {
    pub fn find_at_path(&self, path: &Path) -> Option<&PreviewNode> {
        let buf = normalize(path);
        buf.components()
            .filter(|c| matches!(c, Component::Normal(_)))
            .fold(Some(self), |current_node_opt, item|
            current_node_opt
                .and_then(|current_node| current_node.children())
                .and_then(|children|
                    children.iter().find(|n| n.name() == item.as_os_str())
                )
        )
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use std::{ ffi::OsStr, path::Component };
    use super::*;

    #[test]
    fn test_find_at_path() {
        let node_c = PreviewNode::new_file(OsStr::new("C"), None);
        let node_b = PreviewNode::new_directory_with_children(OsStr::new("B"), vec![node_c.clone()]);
        let node_a = PreviewNode::new_file(OsStr::new("A"), None);

        let node = PreviewNode::new_directory_with_children(Component::RootDir.as_os_str(),
            vec![
                node_a.clone(),
                node_b.clone()
            ]
        );

        assert_eq!(&node, node.find_at_path(&Path::new("/")).unwrap());
        assert_eq!(None, node.find_at_path(&Path::new("/DO/NOT/EXISTS")));

        assert_eq!(&node_a, node.find_at_path(&Path::new("/A")).unwrap());
        assert_eq!(&node_b, node.find_at_path(&Path::new("/B")).unwrap());
        assert_eq!(&node_c, node.find_at_path(&Path::new("/B/C")).unwrap());

        assert_eq!(&node_a, node.find_at_path(&Path::new("A")).unwrap());
        assert_eq!(&node_b, node.find_at_path(&Path::new("B")).unwrap());
        assert_eq!(&node_c, node.find_at_path(&Path::new("B/C")).unwrap());

        assert_eq!(&node_a, node.find_at_path(&Path::new("/./././A")).unwrap());
        assert_eq!(&node_a, node.find_at_path(&Path::new("/B/../A")).unwrap());
        assert_eq!(&node_a, node.find_at_path(&Path::new("/B/../B/../A")).unwrap());

        assert_eq!(None, node_b.find_at_path(&Path::new("B")));
        assert_eq!(&node_c, node_b.find_at_path(&Path::new("C")).unwrap());
    }
}
