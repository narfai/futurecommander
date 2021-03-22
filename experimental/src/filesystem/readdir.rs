use std::{
    path::{ PathBuf, Path },
    fs::DirEntry as FsDirEntry
};

use crate::{
    Result,
    preview::node::Node
};

use super::{ FileTypeExt, DirEntry };

pub struct ReadDir {
    path: PathBuf,
    nodes: Vec<Node>,
    state: ReadDirState
}

pub enum ReadDirState {
    Uninitialized,
    Real(Vec<FsDirEntry>),
    Virtual,
    Terminated
}

impl ReadDir {
    pub fn new(path: &Path, nodes: Vec<Node>) -> Self {
        ReadDir {
            nodes,
            path: path.to_path_buf(),
            state: ReadDirState::Uninitialized
        }
    }
}

impl Iterator for ReadDir {
    type Item = Result<DirEntry>;
    fn next(&mut self) -> Option<Result<DirEntry>> {
        use ReadDirState::*;
        match &mut self.state {
            Uninitialized => {
                self.state = Real(
                    match self.path.read_dir() {
                        Ok(read_dir) =>
                            match read_dir.filter(
                                |entry_result|
                                    match entry_result {
                                        Ok(entry) => self.nodes.iter().find(
                                            |node|node.name() == &entry.file_name()
                                        ).is_none(),
                                        Err(err) => true
                                    }
                            ).collect() {
                                Ok(dir_entries) => dir_entries,
                                Err(err) => return Some(Err(err.into()))
                            },
                        Err(err) => return Some(Err(err.into()))
                    }
                );

                None
            },
            Real(dir_entries) => {
                match dir_entries.pop() {
                    Some(entry) => match entry.file_type() {
                        Ok(file_type) => Some(
                            file_type.into_virtual_file_type().and_then(
                                |virtual_file_type| Ok(DirEntry::new(&entry.path(), entry.file_name(), virtual_file_type))
                            )
                        ),
                        Err(err) => Some(Err(err.into()))
                    },
                    None => {
                        self.state = Virtual;
                        self.next()
                    }
                }
            },
            Virtual => {
                match self.nodes.pop() {
                    Some(node) => Some(
                        node.into_virtual_file_type()
                            .and_then(|virtual_file_type|
                                Ok(  //TODO DirEntryExt (&Path, &Node) DirEntry
                                    DirEntry::new(
                                        &self.path.join(node.name()),
                                        node.name().clone(),
                                        virtual_file_type
                                    )
                                )
                            )
                        )
                    ,
                    None => {
                        self.state = Terminated;
                        self.next()
                    }
                }
            },
            Terminated => None
        }
    }
}
