use std::{
    path::{ PathBuf, Path },
    ffi::{ OsString },
    result,
    io
};

use crate::{
    Result,
    preview::Node
};

use self::super::{
    FileSystemError,
    Metadata,
    FileType,
    FileTypeExt
};

use std::fs::DirEntry as FsDirEntry;

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
                        Ok(file_type) => match file_type.into_virtual_file_type() {
                            Ok(virtual_file_type) => Some(Ok(DirEntry::new(&entry.path(), entry.file_name(), virtual_file_type))), //TODO DirEntryExt FsDirEntry -> DirEntry
                            Err(err) => Some(Err(err.into()))
                        }
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
                    Some(node) => match node.into_virtual_file_type() {
                        Ok(virtual_file_type) => Some(Ok(DirEntry::new(&self.path.join(node.name()), node.name().clone(), virtual_file_type))), //TODO DirEntryExt (&Path, &Node) DirEntry
                        Err(err) => Some(Err(err.into()))
                    },
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

pub struct DirEntry {
    path: PathBuf,
    name: OsString,
    metadata: Metadata
}

impl DirEntry {
    pub fn new(path: &Path, name: OsString, file_type: FileType) -> Self {
        DirEntry {
            path: path.to_path_buf(),
            name,
            metadata: Metadata::new(file_type)
        }
    }

    pub fn path(&self) -> PathBuf { self.path.to_path_buf() }
    pub fn metadata(&self) -> Result<Metadata> { Ok(self.metadata.clone()) }
    pub fn file_type(&self) -> Result<FileType> { Ok(self.metadata.file_type()) }
    pub fn file_name(&self) -> OsString { self.name.clone() }
}

