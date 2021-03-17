use crate::{
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
    pub fn apply(operation: &Operation) -> Result((), NodeError) {
        match operation {
            Copy(source, destination),
            Rename(source, destination),
            RemoveFile(path),
            RemoveDir(path),
            RemoveDirAll(path),
            CreateDirAll(path),
            CreateFile(path)
        }
        Ok(())
    }
}