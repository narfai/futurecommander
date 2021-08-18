/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2021 François CADEILLAN, Damien JORIS
 */

fn main() {
    println!("Hello, world!");
}

/**
 * STD -> INNER EDGE ( INPUT ) -> ( BUSINESS -> STRING ) -> INNER EDGE ( OUTPUT )
 * STD -> Iterator<& dyn DirEntry> -> LIB ( DOMAIN ) -> Iterator<& dyn Node> -> SHELL / UX
 */
use std::path::Path;

// EDGE
pub trait DirEntry {
    fn file_name(&self) -> &str;
    fn is_dir(&self) -> bool;
}

pub trait NodeTrait {
    fn get_children() -> Vec<Node>;
}

trait  {
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

    fn read_nodes_at_path(fs: &dyn FileSystem, path: &Path) -> Vec<Node> {
        return fs
            .read_dir(path)
            .iter()
            .map(|&dir_entry| Node::from(dir_entry))
            .collect();
    }

    #[test]
    fn the_test() {
        struct EmptyFileSystemMock;

        impl FileSystem for EmptyFileSystemMock {
            fn read_dir(&self, path: &Path) -> Vec<&dyn DirEntry> {
                return vec![];
            }
        }

        let fs = EmptyFileSystemMock;

        let node_list = read_nodes_at_path(&fs, Path::new("/A/B/C"));

        /*
            ROOT
                A (Dossier)
                B
                C
                D
                    J
                        F (Fichier)
                    K
                E

            ON ASSUME QU'ON NE GERE PAS LA CONCURRENCE ( édition du fs simultanément a l'utilisation de futurecommander )
            DADOU => LA LECTURE DOIT ETRE FAITE UNE FOIS AU DEBUT AU DEMARRAGE DE L'APPLICATION
            TUX => TROP DE META DONNEES POTENTIELLES => TROP DE LECTURE DISQUE !
            Solution :
                Représente en mémoire :
                    - Les nodes affectés par des opérations (a prevoir une possible serialization)
                    - Tout leur parent recursivement jusqu'a la root
                Si l'utilisateur visualise un node qui n'est pas stocké en mémoire => fallback sur le système de fichier

            Mais pour le MVP :
                - stocker tout en mémoire au démarrage
                - potentiellement lancer le logiciel "chrooté" dans un sous-dossier
                - faire des bench
                - faire une configuration minimale

            Point du jour session 7
                - recupérer un noeud vide
                - faire une interface

            "shell first"
                -> commencer l'implémentation du shell
        */

        assert!(node_list.len() == 0);
    }
    //preview extends std::fs
    //override read_dir() -> comportement de preview
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
