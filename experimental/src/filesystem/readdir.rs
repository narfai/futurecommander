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

    fn _initialize(&mut self) -> Option<Result<DirEntry>>{
        self.path.read_dir()
            .and_then(|read_dir|
                read_dir.filter(|entry_result|
                    entry_result.as_ref().map_or(
                        true,
                        |entry |
                            self.nodes.iter().find(|node|
                                node.name() == &entry.file_name()
                            ).is_none()
                    )
                ).collect()
            )
            .map(|children| {
                self.state = ReadDirState::Real(children);
                self.next()
            })
            .unwrap_or_else(|err| Some(Err(err.into())))
    }

    fn _next_real(&mut self, entry: Option<FsDirEntry>) -> Option<Result<DirEntry>> {
        entry.map(|entry| entry.file_type()
                .map_err(|err| err.into())
                .and_then(|file_type| file_type.into_virtual_file_type())
                .map(|virtual_file_type| DirEntry::new(&entry.path(), entry.file_name(), virtual_file_type))
            ).or_else(|| {
                self.state = ReadDirState::Virtual;
                self.next()
            })
    }

    fn _next_virtual(&mut self) -> Option<Result<DirEntry>> {
        self.nodes.pop()
            .map(|node|
                node.into_virtual_file_type()
                    .map(|virtual_file_type|
                        DirEntry::new(
                            &self.path.join(node.name()),
                            node.name().clone(),
                            virtual_file_type
                        )
                    )
            ).or_else(|| {
                self.state = ReadDirState::Terminated;
                self.next()
            })
    }
}

impl Iterator for ReadDir {
    type Item = Result<DirEntry>;
    fn next(&mut self) -> Option<Result<DirEntry>> {
        use ReadDirState::*;
        match &mut self.state {
            Uninitialized => self._initialize(),
            Real(dir_entries) =>
                dir_entries.pop().map(|entry| entry.file_type()
                    .map_err(|err| err.into())
                    .and_then(|file_type| file_type.into_virtual_file_type())
                    .map(|virtual_file_type| DirEntry::new(&entry.path(), entry.file_name(), virtual_file_type))
                ).or_else(|| {
                    self.state = ReadDirState::Virtual;
                    self.next()
                }),
            Virtual => self._next_virtual(),
            Terminated => None
        }
    }
}
