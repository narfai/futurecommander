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
use std::path::Component;


#[derive(Eq, Debug)]
pub struct Node {
    children: HashSet<Node>,
    name: OsString
}

impl Node {
    pub fn new(name: &str) -> Node {
        Node {
            children: HashSet::new(),
            name: name.into()
        }
    }

    pub fn name(&self) -> &OsString {
        &self.name
    }

    pub fn insert(&mut self, node: Node) -> Result<(), NodeError>{
        if self.children.contains(&node) {
            Err(NodeError::Custom(String::from("Cannot be inserted")))
        } else {
            self.children.insert(node);
            Ok(())
        }
    }

    pub fn find(&self, path: &Path) -> Option<&Node> {
        let mut components = path.components();
        if let Some(component) = components.next() {
            match &component {
                Component::ParentDir => {
                    if let Some(parent_path) = components.as_path().parent() {
                        return self.find(parent_path);
                    }
                },
                Component::Normal(os_str) => {
                    let sub_path = components.as_path();
                    let mut sub_path_components = sub_path.components();
                    match sub_path_components.next() {
                        Some(Component::ParentDir) => {
                            return self.find(sub_path_components.as_path());
                        }
                        _ => {}
                    }
                    for node in &self.children {
                        if os_str == node.name() {
                            if components.next().is_none() {
                                return Some(node);
                            } else if let Some(sub_node) = node.find(sub_path) {
                                return Some(sub_node);
                            }
                        }
                    }
                },
                Component::CurDir | Component::Prefix(_) | Component::RootDir => {
                    let sub_path = components.as_path();
                    if ! components.next().is_none() {
                        return self.find(sub_path)
                    } else if component.as_os_str() == self.name() {
                        return Some(&self);
                    }
                }
            }
        }
        None
    }
}

#[test]
fn test_find_node() {
    let mut node = Node::new("/");
    let node_a = Node::new("A");
    let mut node_b = Node::new("B");
    let node_c = Node::new("C");


    node_b.insert(node_c).unwrap();
    node.insert(node_a).unwrap();
    node.insert(node_b).unwrap();

    assert_eq!(Some(&Node::new("A")), node.find(&Path::new("A")));
    assert_eq!(Some(&Node::new("A")), node.find(&Path::new("/./././A")));
    assert_eq!(Some(&Node::new("A")), node.find(&Path::new("B/../A")));
    assert_eq!(Some(&Node::new("A")), node.find(&Path::new("B/../B/../A")));

    assert_eq!(Some(&Node::new("C")), node.find(&Path::new("B/C")));
    assert_eq!(Some(&Node::new("C")), node.find(&Path::new("/B/C")));
    assert_eq!(Some(&Node::new("C")), node.find(&Path::new("./B/C")));

    assert_eq!(Some(&Node::new("/")), node.find(&Path::new("/")));
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.name.eq(&other.name)
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}


// ================================================================= //

fn main() {
    println!("Hello, world!");
}
