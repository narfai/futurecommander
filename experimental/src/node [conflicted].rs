// ================================================================= //
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum NodeError {
    IoError(io::Error),
    Custom(String),
}

impl From<io::Error> for NodeError {
    fn from(error: io::Error) -> Self {
        NodeError::IoError(error)
    }
}


impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeError::IoError(error) => write!(f, "I/O error {}", error),
            NodeError::Custom(s) => write!(f, "Custom error {}", s),
        }
    }
}

impl error::Error for NodeError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            NodeError::IoError(err) => Some(err),
            _ => None
        }
    }
}

// ================================================================= //

/**
* Thanks to ThatsGobbles ( https://github.com/ThatsGobbles ) for his solution : https://github.com/rust-lang/rfcs/issues/2208
* This code will be removed when os::make_absolute will be marked as stable
*/
pub fn normalize(p: &Path) -> PathBuf {
    let mut stack: Vec<Component<'_>> = vec![];

    for component in p.components() {
        match component {
            Component::CurDir => {},
            Component::ParentDir => {
                match stack.last().cloned() {
                    Some(c) => {
                        match c {
                            Component::Prefix(_) => { stack.push(component); },
                            Component::RootDir => {},
                            Component::CurDir => { unreachable!(); },
                            Component::ParentDir => { stack.push(component); },
                            Component::Normal(_) => { let _ = stack.pop(); }
                        }
                    },
                    None => { stack.push(component); }
                }
            },
            _ => { stack.push(component); },
        }
    }

    if stack.is_empty() {
        return PathBuf::from(".");
    }

    let mut norm_path = PathBuf::new();

    for item in &stack {
        norm_path.push(item);
    }

    norm_path
}
// ================================================================= //
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;
use std::path::Component;
use std::iter;

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
    id: u128
}

impl Eq for Node {}

impl Node {
    pub fn new_directory(id: u128, name: &str) -> Node {
        Node {
            kind: Kind::Directory(HashSet::new()),
            name: name.into(),
            id
        }
    }

    pub fn new_file(id: u128, name: &str, source: Source) -> Node {
        Node {
            kind: Kind::File(source),
            name: name.into(),
            id
        }
    }

    pub fn new_symlink(id: u128, name: &str, path: &Path) -> Node {
        Node {
            kind: Kind::Symlink(path.to_path_buf()),
            name: name.into(),
            id
        }
    }

    pub fn name(&self) -> &OsString {
        &self.name
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

    pub fn insert(&mut self, node: Node) -> Result<&mut Self, NodeError>{
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

    pub fn inserts(&mut self, nodes: impl Iterator<Item = Node>) -> Result<&mut Self, NodeError> {
        for node in nodes {
            self.insert(node)?;
        }
        Ok(self)
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
                iter::once(NodeItem { parent_path, child: &self } )
                    .chain(
                        children.iter()
                            .map(move |n| n._iter(new_parent_path.clone()))
                            .flatten()
                    )
            )
        } else {
            Box::new(iter::once(NodeItem { parent_path, child: &self } ))
        }
    }



    //Synchronize approach is the best in regards of providing best effort against unwatched filesystem changes
    //Have to output a generator of operations over the fs
    //Next try with :
    // - Timestamps
    // - Vfs file source references
    // - Vfs source tracking
    // - Real time application of operations over filesystem
    // - Generate operations in order to let guard decision
    // To represent :
    // Deletion has to happen _in order_ because it have warrant about directory's children unicity : it frees a name's slot in a directory
    // Copy also have that warraant : it _reserve_ a name's slot in a directory
    // Move combines both : it frees a name's slot in a directory and reserve one into another OR THE SAME !
}

#[derive(Debug)]
pub struct NodeItem<'a>{
    parent_path: PathBuf,
    child: &'a Node
}

impl <'a>NodeItem<'a> {
    pub fn parent_path(&self) -> &Path {
        &self.parent_path
    }

    pub fn path(&self) -> PathBuf {
        self.parent_path.join(self.child.name())
    }

    pub fn node(&self) -> &Node {
        self.child
    }

    fn is_contained_by(&self, path: &Path) -> bool {
        for ancestor in self.path().ancestors() {
            if path == ancestor {
                return true;
            }
        }
        false
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
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

    fn assert_two_errors_equals(left: &impl error::Error, right: &impl error::Error) {
        assert_eq!(format!("{}", left), format!("{}", right))
    }

    #[test]
    fn iter_recursively() {
        let node = *Node::new_directory(0, "/")
            .inserts(vec![
                Node::new_file(1, "A", Source::Touch),
                *Node::new_directory(2, "B").clone().insert(
                    Node::new_file(3, "C", Source::Touch).clone()
                ).unwrap()
            ]).unwrap();

        let collection : Vec<PathBuf>= node.iter().map(|item| item.path()).collect();
        collection.contains(&PathBuf::from("/"));
        collection.contains(&PathBuf::from("/A"));
        collection.contains(&PathBuf::from("/B"));
        collection.contains(&PathBuf::from("/B/C"));
    }

    #[test]
    fn test_cant_insert_same_node_twice(){
        let mut node = Node::new_directory(0, "/");
        node.insert(Node::new_file(1, "A", Source::Touch)).unwrap();
        assert_two_errors_equals(
            &NodeError::Custom(String::from("Cannot be inserted")),
            &node.insert(Node::new_file(1, "A", Source::Touch)).err().unwrap()
        );
    }

    #[test]
    fn test_contains_same_node_name(){
        let mut node = Node::new_directory(0, "/");
        node.insert(Node::new_file(1, "A", Source::Touch)).unwrap();
        assert!(node.contains("A").unwrap())
    }

    #[test]
    fn test_find_node() {
        let mut node = Node::new_directory(0, "/");
        let node_a = Node::new_file(1, "A", Source::Touch);
        let mut node_b = Node::new_directory(2, "B");
        let node_c = Node::new_file(3, "C", Source::Touch);

        node.inserts(vec![
            node_a.clone(),
            *node_b.clone().insert(
                node_c.clone()
            ).unwrap()
        ].iter()).unwrap();

        assert_eq!(&node_a, node.find(&Path::new("/A")).unwrap().node());
        assert_eq!(&node_a, node.find(&Path::new("/./././A")).unwrap().node());
        assert_eq!(&node_a, node.find(&Path::new("/B/../A")).unwrap().node());
        assert_eq!(&node_a, node.find(&Path::new("/B/../B/../A")).unwrap().node());

        assert_eq!(&node_c, node.find(&Path::new("/B/C")).unwrap().node());
        assert_eq!(&node_c, node.find(&Path::new("/B/C")).unwrap().node());
        assert_eq!(&node_c, node.find(&Path::new("/B/C")).unwrap().node());
    }

    #[test]
    fn file_dir_interversion() {
        /*
        FROM
        ├── A (Directory)
        │   ├── D (File)
        │   └── E (File)
        └── C (File)

        mv A Z
        mv C A
        mv Z C

        TO
        ├── A (File)
        └── C (Directory)
            ├── D (File)
            └── E (File)
        */
    }

    #[test]
    fn file_file_interversion() {
        /*
        FROM
        ├── A (File) "A"
        └── C (File) "C"

        mv A Z
        mv C A
        mv Z C

        TO
        ├── A (File) "C"
        └── C (File) "A"
        */
    }

    #[test]
    fn dir_dir_interversion() {
        /*
        FROM
        ├── A (Directory)
        │   ├── D (File)
        │   └── E (File)
        └── B (Directory)
            ├── F (File)
            └── G (File)

        mv A Z
        mv B A
        mv Z B

        TO
        ├── A (Directory)
        │   ├── F (File)
        │   └── G (File)
        └── B (Directory)
            ├── D (File)
            └── E (File)
        */
    }

    #[test]
    fn multi_level_interversion() {
        /*
        FROM
        ├── A (Directory)
        │   ├── D (File)
        │   └── E (File)
        └── B (Directory)
            ├── F (File)
            └── G (File)

        mv A B/A
        cp B A

        TO
        ├── A (Directory)
        │   ├── A (Directory)
        │   │   ├── D (File)
        │   │   └── E (File)
        │   ├── F (File)
        │   └── G (File)
        └── B (Directory)
            ├── A (Directory)
            │   ├── D (File)
            │   └── E (File)
            ├── F (File)
            └── G (File)

        */
    }

    #[test]
    fn copy_then_delete_then_create() {
        /*
        FROM
        └── A (Directory)
            ├── D (File)
            └── E (File)

        cp A B
        rm A
        touch A

        TO
        ├── B (Directory)
        │   ├── F (File)
        │   └── G (File)
        └── A (File)
        */
    }


}
