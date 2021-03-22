use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    ffi::{ OsString, OsStr },
    path::{ Path, PathBuf, Components, Component },
    iter
};

use crate::{
    path::normalize,
    FileSystemError,
    Result
};

use super::{
    kind::Kind
};

#[derive(Debug, Clone)]
pub struct Node {
    kind: Kind,
    name: OsString
}

impl Default for Node {
    fn default() -> Self {
        Node::new_directory("/")
    }
}

impl Eq for Node {}

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

    pub fn new_deleted(name: &str) -> Node {
        Node {
            kind: Kind::Deleted,
            name: name.into()
        }
    }

    pub fn name(&self) -> &OsString {
        &self.name
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn is_deleted(&self) -> bool {
        matches!(self.kind, Kind::Deleted)
    }

    pub fn is_directory(&self) -> bool {
        if let Kind::Directory(_) = self.kind {
            true
        } else {
            false
        }
    }

    pub fn source(&self) -> Option<&Path> {
        match &self.kind {
            Kind::File(source) => source.as_ref().and_then(|src| Some(src.as_path())),
            _ => None
        }
    }

    pub fn contains(&self, name: &str) -> Result<bool> {
        if let Kind::Directory(children) = &self.kind {
            for child in children {
                if child.name() == name {
                    return Ok(true);
                }
            }
            Ok(false)
        } else {
            Err(FileSystemError::Custom("Not a Directory".into()))
        }
    }

    pub fn insert(mut self, node: Node) -> Result<Self>{
        if let Kind::Directory(children) = &mut self.kind {
            if children.contains(&node){
                Err(FileSystemError::Custom(String::from("Cannot be inserted")))
            } else {
                children.insert(node);
                Ok(self)
            }
        } else {
            Err(FileSystemError::Custom("Not a Directory".into()))
        }
    }

    pub fn insert_at(&mut self, target_parent_path: &Path, node: &Node) -> Result<&mut Self> {
        *self = self.build(
            &|parent_path, name, kind| {
                if parent_path.join(&name) == target_parent_path {
                    if let Kind::Directory(children) = kind {
                        if children.contains(node) {
                            Err(FileSystemError::Custom(String::from("Cannot be inserted")))
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
                        Err(FileSystemError::Custom("Not a Directory".into()))
                    }
                } else {
                    Ok(Node { name, kind })
                }
            }
        )?;
        Ok(self)
    }

    pub fn filter<P>(&mut self, predicate: P) -> Result<&mut Self>
    where P: Fn(&Path, &Node) -> bool  {
        *self = self.build(
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
        )?;
        Ok(self)
    }

    pub fn build<P>(&self, builder: &P) -> Result<Node>
    where P: Fn(PathBuf, OsString, Kind) -> Result<Node>  {
        let parent_path = PathBuf::from(self.name());
        Node::_build(self.kind.clone(), self.name.clone(), builder, parent_path)
    }


    pub fn find<P>(&self, predicate: P) -> Option<(PathBuf, &Node)>
    where P: Fn(&Path, &Node) -> bool {
        for (path, node) in self.iter().skip(1) {
            if predicate(&path, node){
                return Some((path, node))
            }
        }
        None
    }

    pub fn find_at_path(&self, path: &Path) -> Result<Option<&Node>> {
        let buf = normalize(path);
        let components = buf.components();
        self._find_at_path_router(components)
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = (PathBuf, &Node)> + 'a>{
        self._iter(PathBuf::from(self.name()))
    }

    fn _build<P>(kind: Kind, name: OsString, builder: &P, parent_path: PathBuf) -> Result<Node>
    where P: Fn(PathBuf, OsString, Kind) -> Result<Node>  {
        if let Kind::Directory(children) = kind {
            let new_parent_path = parent_path.join(&name);
            builder(
                parent_path,
                name,
                Kind::Directory(children.into_iter()
                    .map(|child| Node::_build(child.kind, child.name, builder, new_parent_path.clone()))
                    .collect::<Result<HashSet<Node>>>()?
                )
            )
        } else { builder(parent_path, name, kind ) }
    }


    fn _find_at_path_router(&self, mut components: Components) -> Result<Option<&Node>> {
        match components.next() {
            Some(Component::RootDir) => self._find_at_path_node(&OsStr::new("/"), components),
            Some(Component::Normal(current)) => self._find_at_path_node(&current, components),
            _ => Ok(None)
        }
    }

    fn _find_at_path_node(&self, name: &OsStr, components: Components) -> Result<Option<&Node>> {
        if let Some(Component::Normal(next)) = components.clone().next() {
            if let Kind::Directory(children) = &self.kind {
                for child in children.iter().filter(|node| node.name == next) {
                    if let Some(node) = child._find_at_path_router(components.clone())? {
                        return Ok(Some(node));
                    }
                }
            }
        } else if self.name == name {
            return Ok(Some(&self))
        }
        Ok(None)
    }

    fn _iter<'a>(&'a self, parent_path: PathBuf) -> Box<dyn Iterator<Item = (PathBuf, &Node)> + 'a>{
        if let Kind::Directory(children) = &self.kind {
            let new_parent_path = parent_path.join(&self.name);
            Box::new(
                iter::once((parent_path, self))
                    .chain(
                        children.iter()
                            .map(move |n| n._iter(new_parent_path.clone()))
                            .flatten()
                    )
            )
        } else {
            Box::new(iter::once((parent_path, self)))
        }
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

        let collection : Vec<PathBuf>= node.iter().map(|(path, _item)| path.to_path_buf()).collect();
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
            &FileSystemError::Custom(String::from("Cannot be inserted")),
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

        assert_eq!(&node_a, node.find_at_path(&Path::new("/A")).unwrap().unwrap());
        assert_eq!(&node_a, node.find_at_path(&Path::new("/./././A")).unwrap().unwrap());
        assert_eq!(&node_a, node.find_at_path(&Path::new("/B/../A")).unwrap().unwrap());
        assert_eq!(&node_a, node.find_at_path(&Path::new("/B/../B/../A")).unwrap().unwrap());
        assert_eq!(&node_c, node.find_at_path(&Path::new("/B/C")).unwrap().unwrap());
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

        assert_eq!(&node_a, node.find_at_path(&Path::new("/A")).unwrap().unwrap());
        assert_eq!(&node_c, node.find_at_path(&Path::new("/B/C")).unwrap().unwrap());

        node.filter(|parent_path, child| &parent_path.join(child.name()) != Path::new("/B")).unwrap();

        assert_eq!(&node_a, node.find_at_path(&Path::new("/A")).unwrap().unwrap());
        assert_eq!(None, node.find_at_path(&Path::new("/B/C")).unwrap());
    }

    #[test]
    fn test_insertion() {
        let node_a = Node::new_directory("A");
        let node_d = Node::new_directory("D");
        let node_h = Node::new_file("H", None);
        let mut node = Node::new_directory("/");

        node.insert_at(&Path::new("/"), &node_a).unwrap()
            .insert_at(&Path::new("/A"), &node_d).unwrap()
            .insert_at(&Path::new("/A/D"), &node_h).unwrap();

        assert_eq!(&node_a, node.find_at_path(&Path::new("/A")).unwrap().unwrap());
        assert_eq!(&node_d, node.find_at_path(&Path::new("/A/D")).unwrap().unwrap());
        assert_eq!(&node_h, node.find_at_path(&Path::new("/A/D/H")).unwrap().unwrap());
    }
}