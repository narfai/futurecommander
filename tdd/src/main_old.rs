/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2021 François CADEILLAN, Damien JORIS
 */

fn main() {
    println!("Hello, world!");
}

use std::fs;
/**
 * STD -> INNER EDGE ( INPUT ) -> ( BUSINESS -> STRING ) -> INNER EDGE ( OUTPUT )
 * STD -> Iterator<& dyn DirEntry> -> LIB ( DOMAIN ) -> Iterator<& dyn Node> -> SHELL / UX
 */
use std::path::Path;

// Implem std

struct StdFileSystem;

impl FileSystem for StdFileSystem {
    //https://doc.rust-lang.org/std/fs/fn.read_dir.html
    fn read_dir(&self, path: &Path) -> &Iterator<Item = &dyn DirEntry> {
        let dir_entry_iterator = fs::read_dir(path).unwrap();
        //https://doc.rust-lang.org/std/fs/struct.DirEntry.html
        &dir_entry_iterator.map(|dir_entry_result| &DirEntryAdapter::new(dir_entry_result.unwrap()))

        /* let mut dir_entries: Vec<&dyn DirEntry> = vec![];

        for dir_entry_result in dir_entry_iterator {
            let toto = DirEntryAdapter::new(dir_entry_result.unwrap());
            dir_entries.push(&toto);
        }

        dir_entries */
    }
}

struct DirEntryAdapter<T> {
    inner: T,
}

impl<T> DirEntryAdapter<T> {
    fn new(inner: T) -> Self {
        DirEntryAdapter { inner }
    }
}

impl DirEntry for DirEntryAdapter<fs::DirEntry> {
    fn file_name(&self) -> String {
        self.inner.file_name().into_string().unwrap()
    }
    fn is_dir(&self) -> bool {
        self.inner.metadata().unwrap().is_dir()
    }
}

// EDGE
pub trait DirEntry {
    fn file_name(&self) -> String;
    fn is_dir(&self) -> bool;
}

pub trait NodeTrait {
    fn get_children() -> Vec<Node>;
}

trait FileSystem {
    fn read_dir(&self, path: &Path) -> Vec<&dyn DirEntry>;
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
    use super::{DirEntry, Directory, FileSystem, Node, NodeKind};
    use std::path::Path;

    // INNER EDGE
    struct DirEntryStub {
        name: String,
        is_dir: bool,
    }

    impl DirEntry for DirEntryStub {
        fn file_name(&self) -> String {
            self.name.clone()
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

    fn read_nodes_at_path(fs: &dyn FileSystem, path: &Path) -> Vec<Node> {
        return fs
            .read_dir(path)
            .iter()
            .map(|&dir_entry| Node::from(dir_entry))
            .collect();
    }

    #[test]
    fn test_that_when_reading_empty_folder_whe_got_empty_nodes() {
        struct EmptyFileSystemMock;

        impl FileSystem for EmptyFileSystemMock {
            fn read_dir(&self, path: &Path) -> Vec<&dyn DirEntry> {
                return vec![];
            }
        }

        let fs = EmptyFileSystemMock;

        let node_list = read_nodes_at_path(&fs, Path::new("/A/B/C"));

        assert!(node_list.len() == 0);
    }
}

/*
Approche mémoire tout stocker
1/ read_dir du vrai dossier et l'afficher
2/ scan du vrai dossier en mémoire ( construire la représentation en mémoire )
    Faire une fonction unitaire pour ajouter un node a la représentation
3/ read_dir du "faux" dossier
*/
