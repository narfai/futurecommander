use std::{
    collections::HashMap,
    path::{ Path, PathBuf },
    fs::{
        remove_dir_all,
        remove_file,
        create_dir_all
    }
};
use crate::{
    error::NodeError,
    node::{ Node, Id, Source, Kind },
    item::{ NodeItem }
};

#[derive(Debug, Clone)]
pub enum Operation {
    CopyFile(PathBuf, PathBuf),
    MoveFile(PathBuf, PathBuf),
    Rename(PathBuf, PathBuf),
    RemoveNode(PathBuf),
    CreateDirAll(PathBuf),
    CreateFile(PathBuf)
}

#[derive(Debug, Clone)]
pub enum OperationGeneratorState<'a> {
    Additions,
    Deletions,
    Add(Box<OperationGeneratorState<'a>>, NodeItem<'a>, Operation),
    Delete(Box<OperationGeneratorState<'a>>, NodeItem<'a>),
    Terminated
}

pub struct OperationGenerator<'a> {
    state: OperationGeneratorState<'a>,
    additions: Vec<NodeItem<'a>>,
    deletions: Vec<NodeItem<'a>>
}

impl <'a>OperationGenerator<'a> {
    fn is_scheduled_for_deletion(&self, path: &Path) -> bool {
        self.deletions.iter().find(|i|
            match i.node().kind() {
                Kind::Deleted => path == i.path(),
                Kind::File(source) => match source {
                    Source::Move(source_path) => path == source_path,
                    _ => false,
                },
                _ => false
            }
        ).is_some()
    }

    fn is_source_of_scheduled_addition(&self, path: &Path) -> bool {
        self.additions.iter().find(|i| { Some(path) == i.source() }).is_some()
    }
}

impl <'a>Into<OperationGenerator<'a>> for &'a Node {
    fn into(self) -> OperationGenerator<'a> {
        let mut additions: Vec<NodeItem> = self.iter().filter(|item| ! item.is_deleted()).collect();
        additions.sort();
        additions.reverse();
        let mut deletions: Vec<NodeItem> = self.iter().filter(|item| item.is_deleted_or_move()).collect();
        deletions.reverse();
        deletions.sort();

        OperationGenerator {
            state: OperationGeneratorState::Additions,
            additions,
            deletions
        }
    }
}

impl <'a>Iterator for OperationGenerator<'a> {
    type Item = Operation;
    fn next(&mut self) -> Option<Self::Item> {
        use OperationGeneratorState::*;
        match &mut self.state {
            Additions => {
                self.state = if let Some(item) = self.additions.pop() {
                    let path = item.path();
                    Add(
                        Box::new(if self.deletions.contains(&item){
                            Delete(
                                Box::new(Additions),
                                item.clone()
                            )
                        } else {
                            Additions
                        }),
                        item.clone(),
                        match item.node().kind() {
                            Kind::Deleted => unreachable!(),
                            Kind::Symlink(_target) => unimplemented!(),
                            Kind::Directory(_) => Operation::CreateDirAll(path),
                            Kind::File(source) => match source {
                                Source::Copy(source_path) | Source::Move(source_path) => {
                                    let source_path_buf = source_path.to_path_buf();
                                    if self.is_scheduled_for_deletion(&source_path_buf) {
                                        if self.is_source_of_scheduled_addition(&source_path_buf) {
                                            Operation::CopyFile(source_path_buf, path)
                                        } else {
                                            self.deletions.retain(|i| source_path_buf != i.path());
                                            if source_path_buf.parent() == path.parent() {
                                                Operation::Rename(source_path_buf, path)
                                            } else {
                                                Operation::MoveFile(source_path_buf, path)
                                            }
                                        }
                                    } else {
                                        Operation::CopyFile(source_path_buf, path)
                                    }
                                },
                                Source::Touch => Operation::CreateFile(path)
                            }
                        }
                    )
                } else { Deletions };
                self.next()
            },
            Deletions => {
                self.state = if let Some(item) = self.deletions.pop() {
                    Delete(Box::new(Deletions), item.clone())
                } else { Terminated };
                self.next()
            },
            Add(next_state, item, operation) => {
                self.additions.retain(|i| item != i);
                let operation_clone = operation.clone();

                self.state = (**next_state).clone();

                Some(operation_clone)
            },
            Delete(next_state, item) => {
                self.deletions.retain(|i| item != i);

                let operation = match item.node().kind() {
                    Kind::File(source) => match source {
                        Source::Move(source_path) => Operation::RemoveNode(source_path.clone()),
                        _ => Operation::RemoveNode(item.path())
                    },
                    _ => Operation::RemoveNode(item.path())
                };

                self.state = (**next_state).clone();

                Some(operation)
            }
            Terminated => None
        }
    }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;
    use std::error;

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

        let mut node = Node::new_directory(0, "/");
        let node_z = Node::new_file(1, "Z", Source::Move(PathBuf::from("/A")));
        let node_a = Node::new_directory(2, "A");
        // let node_d = (3, "D", Source::Move(PathBuf::from("/C")));
        // let node_e = (4, "E", Source::Move(PathBuf::from("/C")));
        let node_c = Node::new_file(5, "C", Source::Move(PathBuf::from("/Z")));

        node = node.insert(node_z.clone()).unwrap()
                .insert(node_a.clone()).unwrap()
                .insert(node_c.clone()).unwrap();

        use crate::operation_generator::{ OperationGenerator };
        let generator : OperationGenerator = (&node).into();
        for op in generator {
            println!("OP {:?}", op);
        }

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