//TODO READ https://doc.rust-lang.org/rust-by-example/mod.html
mod fs {
    //Driven
    use super::domain::{Directory, Node, NodeRepository};
    use std::fs::{DirEntry, ReadDir};
    use std::path::Path;
    pub struct FileSystem;

    impl FileSystem {
        pub fn new() -> Self {
            FileSystem
        }
    }

    //TODO READ https://doc.rust-lang.org/rust-by-example/conversion/from_into.html#from
    //TODO READ https://doc.rust-lang.org/std/convert/trait.From.html
    impl From<ReadDir> for Directory {
        fn from(read_dir: ReadDir) -> Self {
            Directory::new(
                read_dir
                    .map(|dir_entry_result| {
                        let dir_entry = dir_entry_result.unwrap(); //TODO READ https://doc.rust-lang.org/std/result/index.html
                        let name = dir_entry.file_name().into_string().unwrap();
                        let metadata = dir_entry.metadata().unwrap();

                        if metadata.is_dir() {
                            Node::new_directory(&name)
                        } else {
                            Node::new_file(&name)
                        }
                    })
                    .collect(),
            )
        }
    }

    //https://doc.rust-lang.org/std/fs/fn.read_dir.html
    impl NodeRepository for FileSystem {
        fn get_directory(&self, path: &Path) -> Directory {
            path.read_dir().unwrap().into()
        }
    }
}

mod preview {
    //Driven
    use super::domain::{Directory, NodeRepository};
    use std::path::Path;

    pub struct AllInMemoryPreview;

    impl AllInMemoryPreview {
        pub fn from(node_repository: &dyn NodeRepository) -> Self {
            todo!()
        }
    }

    impl NodeRepository for AllInMemoryPreview {
        fn get_directory(&self, path: &Path) -> Directory {
            todo!()
        }
    }
}

mod shell {
    //Driving
    use super::domain::NodeRepository;
    use std::io::Write;
    use std::path::Path;

    //TODO READ https://doc.rust-lang.org/std/io/trait.Write.html
    pub fn ls<W: Write>(out: &mut W, node_repository: &dyn NodeRepository, path: &Path) {
        todo!();
    }
}

mod domain {
    use std::path::Path;

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

    impl Directory {
        pub fn new(children: Vec<Node>) -> Self {
            Directory { children }
        }
    }

    #[derive(PartialEq)]
    pub struct File;

    #[derive(PartialEq)]
    pub enum NodeKind {
        Directory(Directory),
        File(File),
    }

    #[derive(PartialEq)]
    pub struct Node {
        //Aggregate
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
                kind: NodeKind::File(File),
            }
        }
    }

    impl Directory {
        //TODO READ https://doc.rust-lang.org/rust-by-example/trait/impl_trait.html#impl-trait
        pub fn children(&self) -> impl Iterator<Item = &Node> {
            self.children.iter()
        }
    }

    pub trait NodeRepository {
        fn get_directory(&self, path: &Path) -> Directory;
    }
}

fn main() {
    use std::io::Write;
    use std::path::Path;

    use domain::NodeRepository;
    use fs::FileSystem;
    use preview::AllInMemoryPreview;
    use shell::ls;

    let mut stdout = std::io::stdout();

    let fs = FileSystem::new();
    ls(&mut stdout, &fs, &Path::new("/some/where"));

    /*     let preview = AllInMemoryPreview::from(&fs);
    ls(&mut stdout, &preview, &Path::new("/some/where")); */
}
