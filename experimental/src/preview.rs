use std::{
    path:: PathBuf
};

use crate::{
    path::normalize,
    error::NodeError,
    node::{ Node, Kind },
    operation::{ Operation }
};

pub struct Preview {
    root: Node
}

impl Default for Preview {
    fn default() -> Self {
        Preview {
            root: Node::new_directory("/")
        }
    }
}

impl Preview {
    fn _apply(node: &Node, operation: &Operation) -> Result<Node, NodeError> {
        use Operation::*;
        match operation {
            Copy(source, destination) => node.with_inserted_at(
                destination.parent().unwrap(),
                &Node::new_file(destination.file_name().unwrap().to_str().unwrap(), Some(source.clone()))
            ),
            Rename(source, destination) => Ok(node.clone()),
            RemoveDirAll(path) | RemoveDir(path) | RemoveFile(path) => node.filtered(|parent_path, child| &parent_path.join(child.name()) != path),
            CreateDirAll(path) => {
                let mut ancestors : Vec<PathBuf> = path.ancestors().map(|p| p.to_path_buf()).collect();
                ancestors.reverse();
                ancestors.iter().fold(
                    Ok(node.clone()),
                    |acc, path|
                    match acc {
                        Ok(node) => match node.find(|item| item.path() == normalize(path)) {
                            Some(_) => Ok(node),
                            None => node.with_inserted_at(
                                path.parent().unwrap(),
                                &Node::new_directory(path.file_name().unwrap().to_str().unwrap())
                            )
                        },
                        Err(err) => Err(err)
                    }
                )
            },
            CreateDir(path) => node.with_inserted_at(
                path.parent().unwrap(),
                &Node::new_directory(path.file_name().unwrap().to_str().unwrap())
            ),
            CreateFile(path) => node.with_inserted_at(
                path.parent().unwrap(),
                &Node::new_file(path.file_name().unwrap().to_str().unwrap(), None)
            )
        }
    }
    //TODO that method should throw same errors as std::fs fonctions
    pub fn apply(&mut self, operation: &Operation) -> Result<(), NodeError> {
        self.root = Preview::_apply(&self.root, operation)?;

        Ok(())
    }
}