// ================================================================= //

use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    ffi::OsString,
    path::{ Path, PathBuf },
    cmp::Ordering,
    iter
};

use crate::{
    error::NodeError,
    path::normalize,
    item::NodeItem
};

pub type Id = u128;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Source {
    Copy(PathBuf),
    Move(PathBuf),
    Touch
}

#[derive(Debug, Clone)]
pub enum Kind {
    Directory(HashSet<Node>),
    File(Source),
    Symlink(PathBuf),
    Deleted
}

#[derive(Debug, Clone)]
pub struct Node {
    kind: Kind,
    name: OsString,
    id: Id
}

impl Eq for Node {}

impl Node {
    pub fn new_directory(id: Id, name: &str) -> Node {
        Node {
            kind: Kind::Directory(HashSet::new()),
            name: name.into(),
            id
        }
    }

    pub fn new_file(id: Id, name: &str, source: Source) -> Node {
        Node {
            kind: Kind::File(source),
            name: name.into(),
            id
        }
    }

    pub fn new_symlink(id: Id, name: &str, path: &Path) -> Node {
        Node {
            kind: Kind::Symlink(path.to_path_buf()),
            name: name.into(),
            id
        }
    }

    pub fn name(&self) -> &OsString {
        &self.name
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn contains(&self, name: &str) -> Result<bool, NodeError> {
        if let Kind::Directory(children) = &self.kind {
            for child in children {
                if child.name() == name {
                    return Ok(true);
                }
            }
            Ok(false)
        } else {
            Err(NodeError::Custom("Not a Directory".into()))
        }
    }

    pub fn insert(mut self, node: Node) -> Result<Self, NodeError>{
        if let Kind::Directory(children) = &mut self.kind {
            if children.contains(&node){
                Err(NodeError::Custom(String::from("Cannot be inserted")))
            } else {
                children.insert(node);
                Ok(self)
            }
        } else {
            Err(NodeError::Custom("Not a Directory".into()))
        }
    }

    pub fn find<'a>(&'a self, path: &'a Path) -> Option<NodeItem<'a>> {
        for item in self.iter().skip(1) {
            if item.path() == normalize(path){
                return Some(item)
            }
        }
        None
    }

    pub fn is_directory(&self) -> bool {
        if let Kind::Directory(_) = self.kind {
            true
        } else {
            false
        }
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = NodeItem<'a>> + 'a>{
        self._iter(PathBuf::from(self.name()))
    }

    fn _iter<'a>(&'a self, parent_path: PathBuf) -> Box<dyn Iterator<Item = NodeItem<'a>> + 'a>{
        if let Kind::Directory(children) = &self.kind {
            let new_parent_path = parent_path.join(&self.name);
            Box::new(
                iter::once(NodeItem::new(parent_path, &self))
                    .chain(
                        children.iter()
                            .map(move |n| n._iter(new_parent_path.clone()))
                            .flatten()
                    )
            )
        } else {
            Box::new(iter::once(NodeItem::new(parent_path, &self)))
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name().eq(other.name())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name().hash(state);
    }
}


#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;
    use std::error;

    fn assert_two_errors_equals(left: &impl error::Error, right: &impl error::Error) {
        assert_eq!(format!("{}", left), format!("{}", right))
    }

    #[test]
    fn iter_recursively() {
        let node = Node::new_directory(0, "/")
            .insert(Node::new_file(1, "A", Source::Touch)).unwrap()
            .insert(
                Node::new_directory(2, "B")
                    .insert(Node::new_file(3, "C", Source::Touch))
                    .unwrap()
            ).unwrap();

        let collection : Vec<PathBuf>= node.iter().map(|item| item.path()).collect();
        collection.contains(&PathBuf::from("/"));
        collection.contains(&PathBuf::from("/A"));
        collection.contains(&PathBuf::from("/B"));
        collection.contains(&PathBuf::from("/B/C"));
    }

    #[test]
    fn test_cant_insert_same_node_twice(){
        let mut node = Node::new_directory(0, "/");
        node = node.insert(Node::new_file(1, "A", Source::Touch)).unwrap();
        assert_two_errors_equals(
            &NodeError::Custom(String::from("Cannot be inserted")),
            &node.insert(Node::new_file(1, "A", Source::Touch)).err().unwrap()
        );
    }

    #[test]
    fn test_contains_same_node_name(){
        let mut node = Node::new_directory(0, "/");
        node = node.insert(Node::new_file(1, "A", Source::Touch)).unwrap();
        assert!(node.contains("A").unwrap())
    }

    #[test]
    fn test_find_node() {
        let mut node = Node::new_directory(0, "/");
        let node_a = Node::new_file(1, "A", Source::Touch);
        let node_b = Node::new_directory(2, "B");
        let node_c = Node::new_file(3, "C", Source::Touch);

        node = node.insert(node_a.clone()).unwrap()
            .insert(
                node_b.clone().insert(
                    node_c.clone()
                ).unwrap()
        ).unwrap();

        assert_eq!(&node_a, node.find(&Path::new("/A")).unwrap().node());
        assert_eq!(&node_a, node.find(&Path::new("/./././A")).unwrap().node());
        assert_eq!(&node_a, node.find(&Path::new("/B/../A")).unwrap().node());
        assert_eq!(&node_a, node.find(&Path::new("/B/../B/../A")).unwrap().node());

        assert_eq!(&node_c, node.find(&Path::new("/B/C")).unwrap().node());
        assert_eq!(&node_c, node.find(&Path::new("/B/C")).unwrap().node());
        assert_eq!(&node_c, node.find(&Path::new("/B/C")).unwrap().node());
    }
}
