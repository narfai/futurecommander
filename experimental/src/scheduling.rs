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

pub enum Operation {
    Copy(PathBuf, PathBuf),
    Move(PathBuf, PathBuf),
    Rename(PathBuf, PathBuf),
    RemoveFile(PathBuf),
    RemoveDirAll(PathBuf),
    CreateDirAll(PathBuf),
    CreateFile(PathBuf)
}

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

impl <'a>Into<OperationGenerator<'a>> for &'a Node {
    fn into(self) -> OperationGenerator<'a> {
        let mut additions: Vec<NodeItem> = self.iter().filter(|item| ! item.is_deleted()).collect();
        additions.sort();
        additions.reverse();
        let mut deletions: Vec<NodeItem> = self.iter().filter(|item| item.is_deleted()).collect();
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
    type Item = Result<Operation, NodeError>;
    fn next(&mut self) -> Option<Self::Item> {
        use OperationGeneratorState::*;
        match self.state {
            Additions => {
                self.state = if let Some(item) = self.additions.pop() {
                    let path = item.path();
                    let add_state = Add(
                        Box::new(Additions),
                        item,
                        if let Some(source_path) = item.source(){
                            if let Some(source_deletion_item) = self.deletions.iter().find(|i| { source_path == i.path() }) {
                                self.deletions.retain(|i| source_deletion_item != i );
                                Operation::Move(source_path.to_path_buf(), path)
                            } else {
                                Operation::Copy(source_path.to_path_buf(), path)
                            }
                        } else {
                            Operation::CreateFile(path)
                        }
                    );

                    if let Some(deletion_item) = self.deletions.iter().find(|i| { path == i.path() }) {
                        Delete(
                            Box::new(add_state),
                            item
                        )
                    } else { add_state }
                } else { Deletions };
                self.next()
            },
            Deletions => {
                self.state = if let Some(item) = self.deletions.pop() {
                    Delete(Box::new(Deletions), item)
                } else { Terminated };
                self.next()
            },
            Add(next_state, item, operation) => {
                self.state = *next_state;
                self.additions.retain(|&i| item != i);

                Some(Ok(operation))
            },
            Delete(next_state, item) => {
                self.state = *next_state;
                self.deletions.retain(|&i| item != i);

                match item.node().kind() {
                    Kind::Directory(_) => Some(Ok(Operation::RemoveDirAll(item.path()))),
                    _ => Some(Ok(Operation::RemoveFile(item.path())))
                }
            }
            Terminated => None
        }
    }
}