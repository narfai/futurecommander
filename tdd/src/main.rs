/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2021 François CADEILLAN, Damien JORIS
 */

fn main() {
    println!("Hello, world!");
}

// INNER EDGE
pub struct DirEntryBridge {
    name: String, 
    is_dir: bool
}

impl DirEntryBridge {
    pub fn file_name(&self) -> &str {
        &self.name
    }

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }
}

// BUSINESS
#[derive(Debug, Clone)]
pub enum Node {
    Directory {
        name: String
    },
    File {
        name: String
    }
}

impl Node {
    pub fn from(dir_entry: DirEntryBridge) -> Self {
        if dir_entry.is_dir() {
            Node::Directory { name: dir_entry.file_name().to_owned() }
        } else {
            Node::File { name: dir_entry.file_name().to_owned() }
        }
    }
}

//Equivalent de #[derive(Eq)]
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Node::Directory{ name: self_name } => 
                match other {
                    Node::Directory{ name: other_name } => self_name == other_name,
                    Node::File { name: _ } => false 
                },
            Node::File{ name: self_name } => 
                match other {
                    Node::File{ name: other_name } => self_name == other_name,
                    Node::Directory { name: _ }=> false
                }
        }
    }
}

#[test]
fn test_extract_node_from_direntry_is_a_directory() {
    let dir_entry = DirEntryBridge {
        name: String::from("A"),
        is_dir: true
    };

    let expected = Node::Directory {
        name: String::from("A") 
    };
    
    assert!(Node::from(dir_entry) == expected);
}

#[test]
fn test_extract_node_from_direntry_is_a_file() {
    let dir_entry = DirEntryBridge {
        name: String::from("F"),
        is_dir: false
    };

    let expected = Node::File {
        name: String::from("F") 
    };

    assert!(Node::from(dir_entry) == expected);
}