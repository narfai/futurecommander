// ================================================================= //
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum NodeError {
    Custom(String)
}


impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeError::Custom(s) => write!(f, "Custom error {}", s),
        }
    }
}

impl error::Error for NodeError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            _ => None
        }
    }
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

    pub fn find(&self, path: &Path) -> Result<Option<&Node>, NodeError> {
        let mut components = path.components();
        if let Some(component) = components.next() {
            let mut sub_path_components = components.clone();
            if let Some(Component::ParentDir) = sub_path_components.next() {
                return Ok(self.find(sub_path_components.as_path())?);
            }

            if let Node::Directory { name: _, children } = self {
                match &component {
                    Component::ParentDir => {
                        if let Some(parent_path) = components.as_path().parent() {
                            return Ok(self.find(parent_path)?);
                        }
                    },
                    Component::Normal(os_str) => {
                        let sub_path = components.as_path();

                        for node in children {
                            if os_str == node.name() {
                                if components.next().is_none() {
                                    return Ok(Some(node));
                                } else if node.is_directory() {
                                    if let Some(sub_node) = node.find(sub_path)? {
                                        return Ok(Some(sub_node));
                                    }
                                }
                            }
                        }
                    },
                    Component::CurDir | Component::Prefix(_) | Component::RootDir => {
                        let sub_path = components.as_path();
                        if ! components.next().is_none() {
                            return self.find(sub_path);
                        } else if component.as_os_str() == self.name() {
                            return Ok(Some(&self));
                        }
                    }
                }
            } else {
                return Err(NodeError::Custom("Not a Directory".into()));
            }
        }
        Ok(None)
    }

    pub fn is_directory(&self) -> bool {
        if let Node::Directory{ name: _, children:_ } = self {
            true
        } else {
            false
        }
    }

    pub fn items<'a>(&'a self, parent_path: &'a Path) -> Box<dyn Iterator<Item = NodeItem<'a>> + 'a>{
        if let Node::Directory{ name: _, children } = self {
            Box::new(
                children.iter().map(|child| NodeItem {
                    parent_path,
                    child
                })
            )
        } else {
            Box::new(iter::empty())
        }
    }
}

pub struct NodeIntoIterator<'a> {
    path: PathBuf,
    own_iterator: Box<dyn Iterator<Item = &'a Node> + 'a>,
    children_iterator: Box<dyn Iterator<Item = NodeItem<'a>> + 'a>
}

impl <'a>IntoIterator for &'a Node {
    type Item = NodeItem<'a>;
    type IntoIter = NodeIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIntoIterator {
            path: PathBuf::from(self.name()),
            own_iterator: if let Node::Directory { name: _, children } = self {
                Box::new(children.iter())
            } else {
                Box::new(iter::empty())
            },
            children_iterator: Box::new(iter::empty())
        }
    }
}

impl <'a>Iterator for NodeIntoIterator<'a> {
    type Item = NodeItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(child_item) = self.children_iterator.next() {
            Some(child_item)
        } else {
            if let Some(child) = self.own_iterator.next() {
                self.path.push(child.name());
                self.children_iterator = child.items(&'a self.path);
                self.next()
            } else {
                None
            }
        }
    }
}

#[derive(Debug)]
pub struct NodeItem<'a>{
    parent_path: &'a Path,
    child: &'a Node
}

// (&Path, &Node, &Node)

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

        for item in node.it(None) {
            println!("{:?}", item);
        }
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

        assert_eq!(Some(&node_a), node.find(&Path::new("A")).unwrap());
        assert_eq!(Some(&node_a), node.find(&Path::new("/./././A")).unwrap());
        assert_eq!(Some(&node_a), node.find(&Path::new("B/../A")).unwrap());
        assert_eq!(Some(&node_a), node.find(&Path::new("B/../B/../A")).unwrap());

        assert_eq!(Some(&node_c), node.find(&Path::new("B/C")).unwrap());
        assert_eq!(Some(&node_c), node.find(&Path::new("/B/C")).unwrap());
        assert_eq!(Some(&node_c), node.find(&Path::new("./B/C")).unwrap());

        assert_eq!(Some(&node), node.find(&Path::new("/")).unwrap());
    }
}