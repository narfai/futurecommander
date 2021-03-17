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

#[derive(Debug, Clone)]
pub enum Kind {
    Directory(HashSet<Node>),
    File(Option<PathBuf>),
    Symlink(PathBuf)
}

#[derive(Debug, Clone)]
pub struct Node {
    kind: Kind,
    name: OsString
}

impl Eq for Node {}

impl Node {
    pub fn new_directory(name: &str) -> Node {
        Node {
            kind: Kind::Directory(HashSet::new()),
            name: name.into()
        }
    }

    pub fn new_file(name: &str, source: Option<PathBuf>) -> Node {
        Node {
            kind: Kind::File(source),
            name: name.into()
        }
    }

    pub fn new_symlink(name: &str, path: &Path) -> Node {
        Node {
            kind: Kind::Symlink(path.to_path_buf()),
            name: name.into()
        }
    }

    pub fn name(&self) -> &OsString {
        &self.name
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

    pub fn with_inserted_at(self, target_parent_path: &Path, node: &Node) -> Result<Node, NodeError> {
        self.build(
            &|parent_path, name, kind| {
                if parent_path.join(&name) == target_parent_path {
                    if let Kind::Directory(children) = kind {
                        if children.contains(node) {
                            Err(NodeError::Custom(String::from("Cannot be inserted")))
                        } else {
                            Ok(Node {
                                kind: Kind::Directory(
                                    children.into_iter()
                                    .chain(iter::once(node.clone()))
                                    .collect()
                                ),
                                name
                            })
                        }
                    } else {
                        Err(NodeError::Custom("Not a Directory".into()))
                    }
                } else {
                    Ok(Node { name, kind })
                }
            }
        )
    }

    pub fn filtered<P>(self, predicate: P) -> Result<Node, NodeError>
    where P: Fn(&Path, &Node) -> bool  {
        self.build(
            &|parent_path, name, kind|
            if let Kind::Directory(children) = kind {
                Ok(Node {
                    kind: Kind::Directory(
                        children.into_iter()
                        .filter(|child| predicate(&parent_path, &child))
                        .collect()
                    ),
                    name
                })
            } else {
                Ok(Node { name, kind })
            }
        )
    }

    pub fn build<'a, P>(self, builder: &'a P) -> Result<Node, NodeError>
    where P: Fn(PathBuf, OsString, Kind) -> Result<Node, NodeError>  {
        let name = PathBuf::from(self.name());
        Node::_build(self.kind, self.name, builder, name)
    }

    fn _build<'a, P>(kind: Kind, name: OsString, builder: &'a P, parent_path: PathBuf) -> Result<Node, NodeError>
    where P: Fn(PathBuf, OsString, Kind) -> Result<Node, NodeError>  {
        if let Kind::Directory(children) = kind {
            let new_parent_path = parent_path.join(&name);
            builder(
                parent_path,
                name,
                Kind::Directory(children.into_iter()
                    .map(|child| Node::_build(child.kind, child.name, builder, new_parent_path.clone()))
                    .collect::<Result<HashSet<Node>, NodeError>>()?
                )
            )
        } else { builder(parent_path, name, kind ) }
    }

    pub fn find<'a, P>(&'a self, predicate: P) -> Option<NodeItem<'a>>
    where P: Fn(&NodeItem<'a>) -> bool {
        for item in self.iter().skip(1) {
            if predicate(&item){
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
        let node = Node::new_directory("/")
            .insert(Node::new_file("A", None)).unwrap()
            .insert(
                Node::new_directory("B")
                    .insert(Node::new_file("C", None))
                    .unwrap()
            ).unwrap();

        let collection : Vec<PathBuf>= node.iter().map(|item| item.path().to_path_buf()).collect();
        collection.contains(&PathBuf::from("/"));
        collection.contains(&PathBuf::from("/A"));
        collection.contains(&PathBuf::from("/B"));
        collection.contains(&PathBuf::from("/B/C"));
    }

    #[test]
    fn test_cant_insert_same_node_twice(){
        let mut node = Node::new_directory("/");
        node = node.insert(Node::new_file("A", None)).unwrap();
        assert_two_errors_equals(
            &NodeError::Custom(String::from("Cannot be inserted")),
            &node.insert(Node::new_file("A", None)).err().unwrap()
        );
    }

    #[test]
    fn test_contains_same_node_name(){
        let mut node = Node::new_directory("/");
        node = node.insert(Node::new_file("A", None)).unwrap();
        assert!(node.contains("A").unwrap())
    }

    #[test]
    fn test_find_node() {
        let mut node = Node::new_directory("/");
        let node_a = Node::new_file("A", None);
        let node_b = Node::new_directory("B");
        let node_c = Node::new_file("C", None);

        node = node.insert(node_a.clone()).unwrap()
            .insert(
                node_b.clone().insert(
                    node_c.clone()
                ).unwrap()
        ).unwrap();

        assert_eq!(&node_a, node.find(|item| item.path() == normalize(&Path::new("/A"))).unwrap().node());
        assert_eq!(&node_a, node.find(|item| item.path() == normalize(&Path::new("/./././A"))).unwrap().node());
        assert_eq!(&node_a, node.find(|item| item.path() == normalize(&Path::new("/B/../A"))).unwrap().node());
        assert_eq!(&node_a, node.find(|item| item.path() == normalize(&Path::new("/B/../B/../A"))).unwrap().node());

        assert_eq!(&node_c, node.find(|item| item.path() == normalize(&Path::new("/B/C"))).unwrap().node());
    }

    #[test]
    fn test_filtered() {
        let mut node = Node::new_directory("/");
        let node_a = Node::new_file("A", None);
        let node_b = Node::new_directory("B");
        let node_c = Node::new_file("C", None);

        node = node.insert(node_a.clone()).unwrap()
            .insert(
                node_b.clone().insert(
                    node_c.clone()
                ).unwrap()
        ).unwrap();

        assert_eq!(&node_a, node.find(|item| item.path() == normalize(&Path::new("/A"))).unwrap().node());
        assert_eq!(&node_c, node.find(|item| item.path() == normalize(&Path::new("/B/C"))).unwrap().node());

        node = node.filtered(|parent_path, child| &parent_path.join(child.name()) != Path::new("/B")).unwrap();
        assert_eq!(&node_a, node.find(|item| item.path() == normalize(&Path::new("/A"))).unwrap().node());
        assert_eq!(None, node.find(|item| item.path() == normalize(&Path::new("/B/C"))));
    }

    #[test]
    fn test_insertion() {
        let node_a = Node::new_directory("A");
        let node_d = Node::new_file("D", None);
        let mut node = Node::new_directory("/")
            .with_inserted_at(&Path::new("/"), &node_a).unwrap()
            .with_inserted_at(&Path::new("/A"), &node_d).unwrap();

        assert_eq!(&node_a, node.find(|item| item.path() == normalize(&Path::new("/A"))).unwrap().node());
        assert_eq!(&node_d, node.find(|item| item.path() == normalize(&Path::new("/A/D"))).unwrap().node());
    }
}