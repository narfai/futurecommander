/*
 * Copyright 2019 François CADEILLAN
 *
 * This file is part of FutureCommander.
 *
 * FutureCommander is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * FutureCommander is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with FutureCommander.  If not, see <https://www.gnu.org/licenses/>.
 */
use std::vec::IntoIter;
use std::cmp::Ordering;
use std::path::{ Path, PathBuf };
use std::ffi::OsStr;
use crate::{
    Kind,
    VirtualFileSystem,
    RealFileSystem,
    OperationError,
    representation::VirtualState,
//    operation::*,
    query::{ VirtualStatus, QueryError }
};

pub trait Entry {
    fn path(&self) -> &Path;
    fn to_path(&self) -> PathBuf;
    fn name(&self) -> Option<&OsStr>;
    fn is_dir(&self) -> bool;
    fn is_file(&self) -> bool;
    fn exists(&self) -> bool;
}

impl Eq for Entry {}

impl Ord for Entry {
    fn cmp(&self, other: &Entry) -> Ordering {
        self.path().cmp(other.path())
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.path().cmp(other.path()))
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Entry) -> bool {
        self.path().eq(other.path())
    }
}


use std::slice::Iter;

impl Entry for &Entry { //TODO find a proper way to do that
    fn path(&self) -> &Path {
        self.path()
    }

    fn to_path(&self) -> PathBuf {
        self.to_path()
    }

    fn name(&self) -> Option<&OsStr> {
        self.name()
    }

    fn is_dir(&self) -> bool {
        self.is_dir()
    }

    fn is_file(&self) -> bool {
        self.is_file()
    }

    fn exists(&self) -> bool { self.exists() }
}

#[derive(Debug, Clone)]
pub struct EntryAdapter<T: Clone>(pub T);

impl <T: Clone> EntryAdapter<T> {
    pub fn into_inner(self) -> T {
        self.0
    }

    pub fn as_inner(&self) -> &T {
        &self.0
    }
}

impl Entry for &EntryAdapter<&Path> {
    fn path(&self) -> &Path {
        self.0
    }

    fn to_path(&self) -> PathBuf {
        self.0.to_path_buf()
    }

    fn name(&self) -> Option<&OsStr> {
        self.0.file_name()
    }

    fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    fn is_file(&self) -> bool {
        self.0.is_file()
    }

    fn exists(&self) -> bool { self.0.exists() }
}
impl Entry for EntryAdapter<&Path> {
    fn path(&self) -> &Path {
        self.0
    }

    fn to_path(&self) -> PathBuf {
        self.0.to_path_buf()
    }

    fn name(&self) -> Option<&OsStr> {
        self.0.file_name()
    }

    fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    fn is_file(&self) -> bool {
        self.0.is_file()
    }

    fn exists(&self) -> bool { self.0.exists() }
}


impl Entry for &EntryAdapter<PathBuf> {
    fn path(&self) -> &Path { self.0.as_path() }

    fn to_path(&self) -> PathBuf { self.0.clone() }

    fn name(&self) -> Option<&OsStr> { self.0.file_name() }

    fn is_dir(&self) -> bool { self.0.is_dir() }

    fn is_file(&self) -> bool { self.0.is_file() }

    fn exists(&self) -> bool { self.0.exists() }
}

impl Entry for EntryAdapter<PathBuf> {
    fn path(&self) -> &Path { self.0.as_path() }

    fn to_path(&self) -> PathBuf { self.0.clone() }

    fn name(&self) -> Option<&OsStr> { self.0.file_name() }

    fn is_dir(&self) -> bool { self.0.is_dir() }

    fn is_file(&self) -> bool { self.0.is_file() }

    fn exists(&self) -> bool { self.0.exists() }
}


impl Entry for &EntryAdapter<VirtualStatus> {
    fn path(&self) -> &Path {
        self.0.as_virtual().as_identity()
    }

    fn to_path(&self) -> PathBuf {
        self.0.as_virtual().to_identity()
    }

    fn name(&self) -> Option<&OsStr> {
        self.path().file_name()
    }

    fn is_dir(&self) -> bool {
        match self.0.as_existing_virtual() {
            Some(identity) =>
                match identity.as_kind() {
                    Kind::Directory => true,
                    _ => false
                },
            None => false
        }
    }

    fn is_file(&self) -> bool {
        match self.0.as_existing_virtual() {
            Some(identity) =>
                match identity.as_kind() {
                    Kind::File => true,
                    _ => false
                },
            None => false
        }
    }

    fn exists(&self) -> bool {
        match self.0.state() {
            VirtualState::Exists
            | VirtualState::ExistsVirtually
            | VirtualState::ExistsThroughVirtualParent
            | VirtualState::Replaced => true,

            VirtualState::NotExists
            | VirtualState::Removed
            | VirtualState::RemovedVirtually => false
        }
    }
}

impl Entry for EntryAdapter<VirtualStatus> {
    fn path(&self) -> &Path {
        self.0.as_virtual().as_identity()
    }

    fn to_path(&self) -> PathBuf {
        self.0.as_virtual().to_identity()
    }

    fn name(&self) -> Option<&OsStr> {
        self.path().file_name()
    }

    fn is_dir(&self) -> bool {
        match self.0.as_existing_virtual() {
            Some(identity) =>
                match identity.as_kind() {
                    Kind::Directory => true,
                    _ => false
                },
            None => false
        }
    }

    fn is_file(&self) -> bool {
        match self.0.as_existing_virtual() {
            Some(identity) =>
                match identity.as_kind() {
                    Kind::File => true,
                    _ => false
                },
            None => false
        }
    }

    fn exists(&self) -> bool {
        match self.0.state() {
            VirtualState::Exists
            | VirtualState::ExistsVirtually
            | VirtualState::ExistsThroughVirtualParent
            | VirtualState::Replaced => true,

            VirtualState::NotExists
            | VirtualState::Removed
            | VirtualState::RemovedVirtually => false
        }
    }
}


#[derive(Debug, Default)]
pub struct EntryCollection<'a, T: 'a>(pub Vec<&'a T>)
    where T: Entry;

impl <'a, T> EntryCollection<'a, T> where T: Entry {
    pub fn contains(&self, node: &Entry) -> bool {
        for owned_node in self.0.iter() {
            if owned_node.path() == node.path() {
                return true;
            }
        }
        false
    }

    pub fn new() -> Self {
        EntryCollection(Vec::new())
    }

    pub fn add(&mut self, node: T) {
        self.0.push(&node)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> Iter<'_, &T> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}


trait WriteableFileSystem: ReadableFileSystem {
    //Write API Interface
    fn create_empty_directory(&mut self, path: &Path) -> Result<(), OperationError>;
    fn create_empty_file(&mut self, path: &Path) -> Result<(), OperationError>;
    fn copy_file_into_directory(&mut self, source: &Path, destination: &Path) -> Result<(), OperationError>;
    fn move_file_into_directory(&mut self, source: &Path, destination: &Path) -> Result<(), OperationError>;
    fn remove_file(&mut self, path: &Path) -> Result<(), OperationError>;
    fn remove_empty_directory(&mut self, path: &Path) -> Result<(), OperationError>;
}

pub struct FileSystemAdapter<T>(pub T);

impl WriteableFileSystem for FileSystemAdapter<VirtualFileSystem> {
    //Write virtual specialization
    fn create_empty_directory(&mut self, path: &Path) -> Result<(), OperationError> { unimplemented!() }
    fn create_empty_file(&mut self, path: &Path) -> Result<(), OperationError> { unimplemented!() }
    fn copy_file_into_directory(&mut self, source: &Path, destination: &Path) -> Result<(), OperationError>{ unimplemented!() }
    fn move_file_into_directory(&mut self, source: &Path, destination: &Path) -> Result<(), OperationError>{ unimplemented!() }
    fn remove_file(&mut self, path: &Path) -> Result<(), OperationError> { unimplemented!() }
    fn remove_empty_directory(&mut self, path: &Path) -> Result<(), OperationError>{ unimplemented!() }
}

impl WriteableFileSystem for FileSystemAdapter<RealFileSystem> {
    //Write real specialization
    fn create_empty_directory(&mut self, path: &Path) -> Result<(), OperationError> { unimplemented!() }
    fn create_empty_file(&mut self, path: &Path) -> Result<(), OperationError> { unimplemented!() }
    fn copy_file_into_directory(&mut self, source: &Path, destination: &Path) -> Result<(), OperationError>{ unimplemented!() }
    fn move_file_into_directory(&mut self, source: &Path, destination: &Path) -> Result<(), OperationError>{ unimplemented!() }
    fn remove_file(&mut self, path: &Path) -> Result<(), OperationError> { unimplemented!() }
    fn remove_empty_directory(&mut self, path: &Path) -> Result<(), OperationError>{ unimplemented!() }
}

trait ReadableFileSystem {
    fn read_dir(&self, path: &Path) -> Result<EntryCollection<&Entry>,QueryError>;
    fn status(&self, path: &Path) -> Result<&Entry, QueryError>;
}

impl ReadableFileSystem for FileSystemAdapter<VirtualFileSystem> {
    //Read virtual specialization
    fn read_dir(&self, path: &Path) -> Result<EntryCollection<&Entry>, QueryError> { unimplemented!() }
    fn status(&self, path: &Path) -> Result<&Entry, QueryError> { unimplemented!() }
}

impl ReadableFileSystem for FileSystemAdapter<RealFileSystem> {
    //Read real specialization
    fn read_dir(&self, path: &Path) -> Result<EntryCollection<&Entry>,QueryError> {
        if ! path.exists() {
            return Err(QueryError::ReadTargetDoesNotExists(path.to_path_buf()))
        }

        if ! path.is_dir() {
            return Err(QueryError::IsNotADirectory(path.to_path_buf()))
        }

        let mut entry_collection = EntryCollection::new();

        match path.read_dir() {
            Ok(results) => {
                for result in results {
                    match result {
                        Ok(result) => entry_collection.add(&EntryAdapter(result.path().as_path())),
                        Err(error) => return Err(QueryError::from(error))
                    };
                }
                Ok(entry_collection)
            },
            Err(error) => Err(QueryError::from(error))
        }
    }

    fn status(&self, path: &Path) -> Result<&Entry, QueryError> {
        Ok(EntryAdapter(path))
    }
}

pub enum AtomicOperation {
    CreateEmptyDirectory(PathBuf),
    CreateEmptyFile(PathBuf),
    CopyFileIntoDirectory(PathBuf, PathBuf),
    MoveFileIntoDirectory(PathBuf, PathBuf),
    RemoveFile(PathBuf),
    RemoveEmptyDirectory(PathBuf)
}

impl AtomicOperation {
    pub fn apply<F: WriteableFileSystem>(self, fs: &mut F) -> Result<(), OperationError> {
        use self::AtomicOperation::*;
        match self {
            CreateEmptyDirectory(path) => fs.create_empty_directory(path.as_path()),
            CreateEmptyFile(path) => fs.create_empty_file(path.as_path()),
            CopyFileIntoDirectory(source, destination) => fs.copy_file_into_directory(source.as_path(), destination.as_path()),
            MoveFileIntoDirectory(source, destination) => fs.move_file_into_directory(source.as_path(), destination.as_path()),
            RemoveFile(path) => fs.remove_file(path.as_path()),
            RemoveEmptyDirectory(path) => fs.remove_empty_directory(path.as_path())
        }
    }
    //TODO apply_at(op position) => makes the index bubble through errors
    //TODO rollback & reverse there
}

pub struct AtomicTransaction(pub Vec<AtomicOperation>);

impl AtomicTransaction {
    pub fn apply<F: WriteableFileSystem>(self, fs: &mut F) -> Result<(), OperationError> {
        for atomic in self.0 {
            atomic.apply(fs)?
        }
        Ok(())
    }

    pub fn add(&mut self, atomic: AtomicOperation) {
        self.0.push(atomic)
    }
}

impl IntoIterator for AtomicTransaction {
    type Item = AtomicOperation;
    type IntoIter = IntoIter<AtomicOperation>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub struct RequestTransaction(pub Vec<Box<dyn Request>>);

pub trait Request {
    fn atomize(&self, fs: &ReadableFileSystem) -> Result<AtomicTransaction, OperationError>;
}

#[derive(Debug, Clone)]
pub struct CopyRequest {
    source: PathBuf,
    destination: PathBuf,
    merge: bool,
    overwrite: bool
}

impl CopyRequest {
    //App
    pub fn new(source: &Path, destination: &Path, merge: bool, overwrite: bool) -> CopyRequest {
        CopyRequest {
            source: source.to_path_buf(),
            destination: destination.to_path_buf(),
            merge,
            overwrite
        }
    }

    pub fn source(&self) -> &Path { self.source.as_path() }
    pub fn destination(&self) -> &Path { self.destination.as_path() }
    pub fn merge(&self) -> bool { self.merge }
    pub fn overwrite(&self) -> bool { self.overwrite }
}

impl Request for CopyRequest {
    fn atomize(&self, fs: &ReadableFileSystem) -> Result<AtomicTransaction, OperationError> {
        //Business
        unimplemented!()
    }
}

fn test(){
    let mut vfs = FileSystemAdapter(VirtualFileSystem::default());
    let request = CopyRequest::new(Path::new("SRC"), Path::new("DST"), false, false);
    request
        .atomize(&vfs)
        .unwrap()
        .apply(&mut vfs)
        .unwrap();
}
//CopyRequest->atomize(ReadableFileSystem) => AtomicTransaction.apply(WriteableFileSystem)
