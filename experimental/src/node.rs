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

#[derive(Eq, Debug, Clone)]
pub enum Node {
    Directory {
        children: HashSet<Node>,
        name: OsString
        // Do a directory may have a optional source as well ?
    },
    File {
        name: OsString,
        source: Source
    },
    Symlink {
        name: OsString,
        path: PathBuf
    },
    Deleted {
        name: OsString
    }
}

impl Node {
    pub fn new_directory(name: &str) -> Node {
        Node::Directory {
            children: HashSet::new(),
            name: name.into()
        }
    }

    pub fn new_file(name: &str, source: Source) -> Node {
        Node::File { source, name: name.into() }
    }

    pub fn new_symlink(name: &str, path: &Path) -> Node {
        Node::Symlink {
            name: name.into(),
            path: path.to_path_buf()
        }
    }

    pub fn name(&self) -> &OsString {
        use Node::*;
        match &self {
            Directory { name, children: _ } => name,
            File { name, source: _ } => name,
            Symlink { name, path: _ } => name,
            Deleted { name } => name
        }
    }

    pub fn contains(&self, name: &str) -> Result<bool, NodeError> {
        if let Node::Directory { name: _, children } = self {
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

    pub fn insert(&mut self, node: Node) -> Result<(), NodeError>{
        if let Node::Directory { name: _, children } = self {
            if children.contains(&node){
                Err(NodeError::Custom(String::from("Cannot be inserted")))
            } else {
                children.insert(node);
                Ok(())
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
        if let Node::Directory{ name: _, children:_ } = self {
            true
        } else {
            false
        }
    }

    pub fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = NodeItem<'a>> + 'a>{
        self._iter(PathBuf::from(self.name()))
    }

    fn _iter<'a>(&'a self, parent_path: PathBuf) -> Box<dyn Iterator<Item = NodeItem<'a>> + 'a>{
        if let Node::Directory{ name, children } = self {
            let new_parent_path = parent_path.join(name);
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
}

#[derive(Debug)]
pub struct NodeItem<'a>{
    parent_path: PathBuf,
    child: &'a Node
}

impl <'a>NodeItem<'a> {
    pub fn path(&self) -> PathBuf {
        self.parent_path.join(self.child.name())
    }

    pub fn node(&self) -> &Node {
        self.child
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


//Synchronize approach is the best in regards of providing best effort against unwatched filesystem changes
//Have to output a generator of operations over the fs
fn apply(node: Node, path: &Path) -> Result<(), NodeError> {
    unimplemented!();
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    fn assert_two_errors_equals(left: &impl error::Error, right: &impl error::Error) {
        assert_eq!(format!("{}", left), format!("{}", right))
    }

    #[test]
    fn iterator_experimental() {
        let mut node = Node::new_directory("/");
        let node_a = Node::new_file("A", Source::Touch);
        let mut node_b = Node::new_directory("B");
        let node_c = Node::new_file("C", Source::Touch);


        node_b.insert(node_c.clone()).unwrap();
        node.insert(node_a.clone()).unwrap();
        node.insert(node_b.clone()).unwrap();

        let collection : Vec<PathBuf>= node.iter().map(|item| item.path()).collect();
        collection.contains(&PathBuf::from("/"));
        collection.contains(&PathBuf::from("/A"));
        collection.contains(&PathBuf::from("/B"));
        collection.contains(&PathBuf::from("/B/C"));
    }

    #[test]
    fn test_cant_insert_same_node_twice(){
        let mut node = Node::new_directory("/");
        node.insert(Node::new_file("A", Source::Touch)).unwrap();
        assert_two_errors_equals(
            &NodeError::Custom(String::from("Cannot be inserted")),
            &node.insert(Node::new_file("A", Source::Touch)).err().unwrap()
        );
    }

    #[test]
    fn test_contains_same_node_name(){
        let mut node = Node::new_directory("/");
        node.insert(Node::new_file("A", Source::Touch)).unwrap();
        assert!(node.contains("A").unwrap())
    }

    #[test]
    fn test_find_node() {
        let mut node = Node::new_directory("/");
        let node_a = Node::new_file("A", Source::Touch);
        let mut node_b = Node::new_directory("B");
        let node_c = Node::new_file("C", Source::Touch);


        node_b.insert(node_c.clone()).unwrap();
        node.insert(node_a.clone()).unwrap();
        node.insert(node_b.clone()).unwrap();

        assert_eq!(&node_a, node.find(&Path::new("/A")).unwrap().node());
        assert_eq!(&node_a, node.find(&Path::new("/./././A")).unwrap().node());
        assert_eq!(&node_a, node.find(&Path::new("/B/../A")).unwrap().node());
        assert_eq!(&node_a, node.find(&Path::new("/B/../B/../A")).unwrap().node());

        assert_eq!(&node_c, node.find(&Path::new("/B/C")).unwrap().node());
        assert_eq!(&node_c, node.find(&Path::new("/B/C")).unwrap().node());
        assert_eq!(&node_c, node.find(&Path::new("/B/C")).unwrap().node());

        //assert_eq!(&node, node.find(&Path::new("/")).unwrap().node());
    }
}