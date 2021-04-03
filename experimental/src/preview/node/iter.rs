/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::{
    iter,
    path::PathBuf
};

use super::{
    PreviewNode
};

fn iter<'a>(node: &'a PreviewNode, parent_path: PathBuf) -> Box<dyn Iterator<Item = (PathBuf, &PreviewNode)> + 'a>{
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

impl PreviewNode {
    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (PathBuf, &PreviewNode)> + 'a> {
        iter(self, PathBuf::from(self.name()))
    }
}