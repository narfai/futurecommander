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
    query::{ VirtualStatus, QueryError, EntryCollection, Entry, EntryAdapter }
};


impl Entry for EntryAdapter<PathBuf> {
    fn path(&self) -> &Path { self.0.as_path() }

    fn to_path(&self) -> PathBuf { self.0.clone() }

    fn name(&self) -> Option<&OsStr> { self.0.file_name() }

    fn is_dir(&self) -> bool { self.0.is_dir() }

    fn is_file(&self) -> bool { self.0.is_file() }

    fn exists(&self) -> bool { self.0.exists() }
}



pub struct FileSystemAdapter<T>(pub T);

impl WriteableFileSystem for FileSystemAdapter<VirtualFileSystem> {
    //Write virtual specialization
    fn create_empty_directory(&mut self, path: &Path) -> Result<(), OperationError> { unimplemented!() }
    fn copy_file_into_directory(&mut self, source: &Path, destination: &Path) -> Result<(), OperationError>{ unimplemented!() }
    fn move_file_into_directory(&mut self, source: &Path, destination: &Path) -> Result<(), OperationError>{ unimplemented!() }
    fn remove_file(&mut self, path: &Path) -> Result<(), OperationError> { unimplemented!() }
    fn remove_empty_directory(&mut self, path: &Path) -> Result<(), OperationError>{ unimplemented!() }
    fn create_empty_file(&mut self, path: &Path) -> Result<(), OperationError> { unimplemented!() }
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

pub trait ReadableFileSystem {
    type Result : Entry;
    fn read_dir(&self, path: &Path) -> Result<EntryCollection<Self::Result>,QueryError>;
    fn status(&self, path: &Path) -> Result<Self::Result, QueryError>;
}

impl ReadableFileSystem for FileSystemAdapter<VirtualFileSystem> {
    type Result = EntryAdapter<VirtualStatus>;
    //Read virtual specialization
    fn read_dir(&self, path: &Path) -> Result<EntryCollection<Self::Result>, QueryError> { unimplemented!() }
    fn status(&self, path: &Path) -> Result<Self::Result, QueryError> { unimplemented!() }
}

impl ReadableFileSystem for FileSystemAdapter<RealFileSystem> {
    type Result = EntryAdapter<PathBuf>;
    //Read real specialization
    fn read_dir(&self, path: &Path) -> Result<EntryCollection<Self::Result>,QueryError> {
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
                        Ok(result) => entry_collection.add(EntryAdapter(result.path())),
                        Err(error) => return Err(QueryError::from(error))
                    };
                }
                Ok(entry_collection)
            },
            Err(error) => Err(QueryError::from(error))
        }
    }

    fn status(&self, path: &Path) -> Result<Self::Result, QueryError> {
        Ok(EntryAdapter(path.to_path_buf()))
    }
}

pub enum Operation {
    CreateEmptyDirectory(PathBuf),
    CreateEmptyFile(PathBuf),
    CopyFileIntoDirectory(PathBuf, PathBuf),
    MoveFileIntoDirectory(PathBuf, PathBuf),
    RemoveFile(PathBuf),
    RemoveEmptyDirectory(PathBuf)
}

impl Operation {
    pub fn apply<F: WriteableFileSystem>(self, fs: &mut F) -> Result<(), OperationError> {
        use self::Operation::*;
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

pub struct Transaction(pub Vec<Operation>);

impl Transaction {
    pub fn apply<F: WriteableFileSystem>(self, fs: &mut F) -> Result<(), OperationError> {
        for atomic in self.0 {
            atomic.apply(fs)?
        }
        Ok(())
    }

    pub fn add(&mut self, atomic: Operation) {
        self.0.push(atomic)
    }
}

impl IntoIterator for Transaction {
    type Item = Operation;
    type IntoIter = IntoIter<Operation>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub struct CommandTransaction(pub Vec<Box<dyn Command<Entry>>>);

pub trait Command<E: Entry> {
    fn atomize(&self, fs: &ReadableFileSystem<Result=E>) -> Result<Transaction, OperationError>;
}

//==================//

#[derive(Debug, Clone)]
pub struct CopyCommand {
    source: PathBuf,
    destination: PathBuf,
    merge: bool,
    overwrite: bool
}

impl CopyCommand {
    //App
    pub fn new(source: &Path, destination: &Path, merge: bool, overwrite: bool) -> CopyCommand {
        CopyCommand {
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

impl <E: Entry> Command<E> for CopyCommand {
    fn atomize(&self, fs: &ReadableFileSystem<Result=E>) -> Result<Transaction, OperationError> {
        //Business
        unimplemented!()
    }
}

fn test(){
    let mut vfs = FileSystemAdapter(VirtualFileSystem::default());
    let Command = CopyCommand::new(Path::new("SRC"), Path::new("DST"), false, false);
    Command
        .atomize(&vfs)
        .unwrap()
        .apply(&mut vfs)
        .unwrap();
}
//CopyCommand->atomize(ReadableFileSystem) => AtomicTransaction.apply(WriteableFileSystem)
