use std::{
    path::{ Component, Path }
};

use crate::{
    Result, FileSystemError,
    path::normalize
};

use super::{
    PreviewNode, PreviewNodeKind
};

fn insert_at_path(parent: &mut PreviewNode, target_parent_path: &Path, node: PreviewNode) -> Result<()> {
    let mut components = target_parent_path.components();
    if let PreviewNodeKind::Directory(children) = &mut parent.kind {
        match components.next() {
            Some(Component::Normal(next)) => {
                if let Some(child) = children.iter_mut().find(|node| node.name() == next) {
                    insert_at_path(child, components.as_path(), node)?;
                } else {
                    let mut child = PreviewNode::new_directory(next);
                    insert_at_path(&mut child,components.as_path(), node)?;
                    children.push(child);
                };
                Ok(())
            },
            Some(_) => insert_at_path(parent, components.as_path(), node),
            None => if children.contains(&node) {
                Err(FileSystemError::Custom("Node already exists".into()))
            } else {
                children.push(node);
                Ok(())
            }
        }
    } else {
        Err(FileSystemError::Custom("Parent node is not a directory".into()))
    }
}

impl PreviewNode {
    pub fn insert_at_path(&mut self, target_parent_path: &Path, node: PreviewNode) -> Result<()> {
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
    use std::{ ffi::OsStr };

    use super::*;

    #[test]
    fn test_insertion_at_root() {
        let node_a = PreviewNode::new_directory(OsStr::new("A"));
        let node_d = PreviewNode::new_directory(OsStr::new("D"));
        let node_h = PreviewNode::new_file(OsStr::new("H"), None);
        let mut node = PreviewNode::default();

        node.insert_at_path(&Path::new("/"), node_a.clone()).unwrap();
        node.insert_at_path(&Path::new("/A"), node_d.clone()).unwrap();
        node.insert_at_path(&Path::new("/A/D"), node_h.clone()).unwrap();

        assert_eq!(&node_a, node.find_at_path(&Path::new("/A")).unwrap());
        assert_eq!(&node_d, node.find_at_path(&Path::new("/A/D")).unwrap());
        assert_eq!(&node_h, node.find_at_path(&Path::new("/A/D/H")).unwrap());
    }

    #[test]
    fn test_dangling_insertion() {
        let mut node = PreviewNode::new_directory(OsStr::new("/"));
        let node_h = PreviewNode::new_file(OsStr::new("H"), None);

        node.insert_at_path(&Path::new("/A/D"), node_h.clone()).unwrap();
        assert_eq!(&node_h, node.find_at_path(&Path::new("/A/D/H")).unwrap());
    }
}
