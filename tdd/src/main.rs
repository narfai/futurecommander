/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2021 François CADEILLAN, Damien JORIS
 */

fn main() {
    println!("Hello, world!");
}

/**
 * STD -> INNER EDGE ( INPUT ) -> ( BUSINESS -> STRING ) -> INNER EDGE ( OUTPUT )
 */

// EDGE
pub trait DirEntry {
    fn file_name(&self) -> &str;
    fn is_dir(&self) -> bool;
}

// BUSINESS
#[derive(PartialEq)]
pub struct Directory {
    children: Vec<Node>,
}

impl Default for Directory {
    fn default() -> Self {
        Directory {
            children: Vec::new(),
        }
    }
}

#[derive(PartialEq)]
pub struct File;

#[derive(PartialEq)]
pub enum NodeKind {
    Directory(Directory),
    File,
}

#[derive(PartialEq)]
pub struct Node {
    name: String,
    kind: NodeKind,
}

impl Node {
    pub fn new_directory(name: &str) -> Self {
        Node {
            name: name.to_owned(),
            kind: NodeKind::Directory(Directory::default()),
        }
    }

    pub fn new_file(name: &str) -> Self {
        Node {
            name: name.to_owned(),
            kind: NodeKind::File,
        }
    }

    pub fn from(dir_entry: &dyn DirEntry) -> Self {
        Node {
            name: dir_entry.file_name().to_owned(),
            kind: if dir_entry.is_dir() {
                NodeKind::Directory(Directory::default())
            } else {
                NodeKind::File
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::{DirEntry, Directory, Node, NodeKind};

    // INNER EDGE
    struct DirEntryStub {
        name: String,
        is_dir: bool,
    }

    impl DirEntry for DirEntryStub {
        fn file_name(&self) -> &str {
            &self.name
        }

        fn is_dir(&self) -> bool {
            self.is_dir
        }
    }

    #[test]
    fn test_extract_node_from_direntry_is_a_directory() {
        let dir_entry = DirEntryStub {
            name: String::from("A"),
            is_dir: true,
        };

        let expected = Node::new_directory(&String::from("A"));

        assert!(Node::from(&dir_entry) == expected);
    }

    #[test]
    fn test_extract_node_from_direntry_is_a_file() {
        let dir_entry = DirEntryStub {
            name: String::from("F"),
            is_dir: false,
        };

        let expected = Node::new_file(&String::from("F"));

        assert!(Node::from(&dir_entry) == expected);
    }

    /*
    DRAFT
    We need to represents the children of a node because we need to display ordered list with ls
    Ls prends un Path en argument
    Node doit être capable de représenter un arbre de donnée
    Node + Path => Node ( enfant )
    */

    // TODO
    // https://doc.rust-lang.org/std/fs/struct.ReadDir.html
    // pub struct ReadDirBridge<'a> {}
    // ReadDirBridge<'a> => NodeIterator<'a>

    // TODO ABSTRAIRE L'OUTPUT
    // Définir l'abstraction d'UI => trait / générique

    // TODO ABSTRAIRE L'INPUT
    // Définir l'abstraction pour wrapper les ReadDir et les DirEntry
    // https://doc.rust-lang.org/std/os/unix/fs/trait.DirEntryExt.html
    // futurecommander::ReadDirExt
    // futurecommander::DirEntryExt

    // Discuter de std::path::Path comme type natif ou outer edge
}
