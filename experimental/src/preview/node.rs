use std::{
    collections::HashSet,
    ffi::{OsStr, OsString},
    hash::{Hash, Hasher},
    path::{Component, Components, Path, PathBuf}
};

use kind::PreviewNodeKind;

use crate::{
    FileSystemError,
    FileType, FileTypeExt,
    Metadata, MetadataExt,
    path::normalize, Result
};

pub mod kind;
mod iter;
mod find_at_path;
mod insert_at_path;
mod retain;
mod tree;

#[derive(Debug, Clone, PartialOrd, Ord)]
pub struct PreviewNode {
    kind: PreviewNodeKind,
    name: OsString
}

impl Default for PreviewNode {
    fn default() -> Self {
        PreviewNode::new_directory(Component::RootDir.as_os_str())
    }
}

impl Eq for PreviewNode {}

impl PartialEq for PreviewNode {
    fn eq(&self, other: &Self) -> bool {
        self.name().eq(other.name())
    }
}

impl Hash for PreviewNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
    }
}

impl PreviewNode {
    pub fn name(&self) -> &OsStr { &self.name }

    pub fn new_directory(name: &OsStr) -> Self {
        PreviewNode {
            kind: PreviewNodeKind::Directory(Vec::new()),
            name: name.to_owned(),
        }
    }

    pub fn new_directory_with_children(name: &OsStr, children: Vec<PreviewNode>) -> Self {
        PreviewNode {
            kind: PreviewNodeKind::Directory(children.into_iter().collect()),
            name: name.to_owned(),
        }
    }

    pub fn new_file(name: &OsStr, source: Option<PathBuf>) -> Self {
        PreviewNode {
            kind: PreviewNodeKind::File(source),
            name: name.to_owned(),
        }
    }

    pub fn new_symlink(name: &OsStr, path: &Path) -> Self {
        PreviewNode {
            kind: PreviewNodeKind::Symlink(path.to_path_buf()),
            name: name.to_owned(),
        }
    }

    pub fn new_deleted(name: &OsStr) -> Self {
        PreviewNode {
            kind: PreviewNodeKind::Deleted,
            name: name.to_owned(),
        }
    }

    pub fn kind(&self) -> &PreviewNodeKind {
        &self.kind
    }

    pub fn source(&self) -> Option<&Path> {
        match &self.kind {
            PreviewNodeKind::File(source) => source.as_ref().map(|src| src.as_path()),
            _ => None
        }
    }

    pub fn children(&self) -> Option<&Vec<PreviewNode>> {
        if let PreviewNodeKind::Directory(children) = &self.kind {
            Some(children)
        } else {
            None
        }
    }

    pub fn is_deleted(&self) -> bool {
        self.kind.is_deleted()
    }
}



impl MetadataExt for &PreviewNode {
    fn into_virtual_metadata(self) -> Result<Metadata> {
        Ok(
            Metadata {
                file_type: self.into_virtual_file_type()?
            }
        )
    }
}

impl FileTypeExt for &PreviewNode {
    fn into_virtual_file_type(self) -> Result<FileType> {
        self.kind.into_virtual_file_type()
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use std::error;

    use super::*;

    fn assert_two_errors_equals(left: &impl error::Error, right: &impl error::Error) {
        assert_eq!(format!("{}", left), format!("{}", right))
    }

    // #[test]
    // fn iter_recursively() {
    //     let node = Node::new_directory("/")
    //         .insert(Node::new_file("A", None)).unwrap()
    //         .insert(
    //             Node::new_directory("B")
    //                 .insert(Node::new_file("C", None))
    //                 .unwrap()
    //         ).unwrap();
    //
    //     let collection : Vec<PathBuf>= node.iter().map(|(path, _item)| path.to_path_buf()).collect();
    //     collection.contains(&PathBuf::from("/"));
    //     collection.contains(&PathBuf::from("/A"));
    //     collection.contains(&PathBuf::from("/B"));
    //     collection.contains(&PathBuf::from("/B/C"));
    // }
    //
    // #[test]
    // fn test_cant_insert_same_node_twice(){
    //     let mut node = Node::new_directory("/");
    //     node = node.insert(Node::new_file("A", None)).unwrap();
    //     assert_two_errors_equals(
    //         &FileSystemError::Custom(String::from("Cannot be inserted")),
    //         &node.insert(Node::new_file("A", None)).err().unwrap()
    //     );
    // }
    //
    // #[test]
    // fn test_find_node() {
    //     let mut node = Node::new_directory("/");
    //     let node_a = Node::new_file("A", None);
    //     let node_b = Node::new_directory("B");
    //     let node_c = Node::new_file("C", None);
    //
    //     node = node.insert(node_a.clone()).unwrap()
    //         .insert(
    //             node_b.clone().insert(
    //                 node_c.clone()
    //             ).unwrap()
    //     ).unwrap();
    //
    //     assert_eq!(&node_a, node.find_at_path(&Path::new("/A")).unwrap().unwrap());
    //     assert_eq!(&node_a, node.find_at_path(&Path::new("/./././A")).unwrap().unwrap());
    //     assert_eq!(&node_a, node.find_at_path(&Path::new("/B/../A")).unwrap().unwrap());
    //     assert_eq!(&node_a, node.find_at_path(&Path::new("/B/../B/../A")).unwrap().unwrap());
    //     assert_eq!(&node_c, node.find_at_path(&Path::new("/B/C")).unwrap().unwrap());
    // }
    //
    // #[test]
    // fn test_filtered() {
    //     let mut node = Node::new_directory("/");
    //     let node_a = Node::new_file("A", None);
    //     let node_b = Node::new_directory("B");
    //     let node_c = Node::new_file("C", None);
    //
    //     node = node.insert(node_a.clone()).unwrap()
    //         .insert(
    //             node_b.clone().insert(
    //                 node_c.clone()
    //             ).unwrap()
    //     ).unwrap();
    //
    //     assert_eq!(&node_a, node.find_at_path(&Path::new("/A")).unwrap().unwrap());
    //     assert_eq!(&node_c, node.find_at_path(&Path::new("/B/C")).unwrap().unwrap());
    //
    //     node.filter(|parent_path, child| parent_path.join(child.name()) != Path::new("/B")).unwrap();
    //
    //     assert_eq!(&node_a, node.find_at_path(&Path::new("/A")).unwrap().unwrap());
    //     assert_eq!(None, node.find_at_path(&Path::new("/B")).unwrap());
    //     assert_eq!(None, node.find_at_path(&Path::new("/B/C")).unwrap());
    // }
    //
    // #[test]
    // fn test_insertion_at_root() {
    //     let node_a = Node::new_directory("A");
    //     let node_d = Node::new_directory("D");
    //     let node_h = Node::new_file("H", None);
    //     let mut node = Node::new_directory("/");
    //
    //     node.insert_at(&Path::new("/"), &node_a).unwrap()
    //         .insert_at(&Path::new("/A"), &node_d).unwrap()
    //         .insert_at(&Path::new("/A/D"), &node_h).unwrap();
    //
    //     assert_eq!(&node_a, node.find_at_path(&Path::new("/A")).unwrap().unwrap());
    //     assert_eq!(&node_d, node.find_at_path(&Path::new("/A/D")).unwrap().unwrap());
    //     assert_eq!(&node_h, node.find_at_path(&Path::new("/A/D/H")).unwrap().unwrap());
    // }
    //
    // #[test]
    // fn test_dangling_insertion() {
    //     let mut node = Node::new_directory("/");
    //     let node_h = Node::new_file("H", None);
    //
    //     node.insert_at(&Path::new("/A/D"), &node_h).unwrap();
    //     assert_eq!(&node_h, node.find_at_path(&Path::new("/A/D/H")).unwrap().unwrap());
    // }
}
