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

#[cfg(test)]
mod test {
    trait DirEntry {
        fn file_name(&self) -> &str;
        fn is_dir(&self) -> bool;
    }

    // INNER EDGE
    pub struct DirEntryStub {
        name: String, 
        is_dir: bool
    }

    impl DirEntry for DirEntryStub {
        fn file_name(&self) -> &str {
            &self.name
        }

        fn is_dir(&self) -> bool {
            self.is_dir
        }
    }

    // BUSINESS
    #[derive(PartialEq)]
    pub enum Node {
        Directory {
            name: String,
            children: Vec<Node> 
        },
        File {
            name: String
        }
    }

    impl Node {
        pub fn from(dir_entry: DirEntryStub) -> Self {
            if dir_entry.is_dir() {
                Node::Directory { name: dir_entry.file_name().to_owned(), children: Vec::new() } 
            } else {
                Node::File { name: dir_entry.file_name().to_owned() }
            }
        }
    }

    #[test]
    fn test_extract_node_from_direntry_is_a_directory() {
        let dir_entry = DirEntryStub {
            name: String::from("A"),
            is_dir: true
        };

        let expected = Node::Directory {
            name: String::from("A"),      
            children: Vec::new()
        };
        
        assert!(Node::from(dir_entry) == expected);
    }

    #[test]
    fn test_extract_node_from_direntry_is_a_file() {
        let dir_entry = DirEntryStub {
            name: String::from("F"),
            is_dir: false
        };

        let expected = Node::File {
            name: String::from("F") 
        };

        assert!(Node::from(dir_entry) == expected);
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
}
