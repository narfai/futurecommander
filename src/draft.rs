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




pub trait UserOperation {
    fn atomics() -> Result<AtomicTransaction, OperationError>;
}

#[derive(Debug, Clone)]
pub struct CopyOperation {
    source: PathBuf,
    destination: PathBuf,
    merge: bool,
    overwrite: bool
}

impl CopyOperation {
    pub fn new(source: &Path, destination: &Path, merge: bool, overwrite: bool) -> CopyOperation {
        CopyOperation {
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

impl UserOperation for CopyOperation {
    fn atomics() -> Result<(), OperationError> {
        //Business stuff
        //TODO needs to uniformize fs read behaviors through query
        unimplemented!()
    }
}



pub struct CreateEmptyFile {
    path: PathBuf
}

pub struct CopyFileIntoDirectory {
    source: PathBuf,
    destination: PathBuf
}

pub struct MoveFileIntoDirectory {
    source: PathBuf,
    destination: PathBuf
}

pub struct RemoveFile {
    //TODO RemoveFile is reversible over vfs but not over fs :/
    path: PathBuf
}

pub struct RemoveEmptyDirectory {
    path: PathBuf
}

//=============================================

pub struct MicroTransaction<F>(pub Vec<Box<dyn MicroOperation<F>>>);//<- Idea for rollback system replace vec by a custom doubled-ended iterator

impl <F>MicroTransaction<F> {
    pub fn apply(&self, fs: &mut F) -> Result<(), OperationError>{
        for operation in self.0.iter() {
            operation.apply(fs)?;
        }
        Ok(())
    }
}

impl <F>IntoIterator for MicroTransaction<F> {
    type Item = Box<dyn MicroOperation<F>>;
    type IntoIter = IntoIter<Box<dyn MicroOperation<F>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}


#[derive(Debug, Clone)]
pub struct CopyOperation {
    source: PathBuf,
    destination: PathBuf,
    merge: bool,
    overwrite: bool
}

impl Operation for CopyOperation {}

impl CopyOperation {
    pub fn new(source: &Path, destination: &Path, merge: bool, overwrite: bool) -> CopyOperation {
        CopyOperation {
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

pub enum PathState {
    File,
    Directory,
    DoesNotExists
}

impl <T: Entry> From<T> for PathState {
    fn from(entry: T) -> Self {
        if entry.exists() {
            if entry.is_dir() {
                PathState::Directory
            } else {
                PathState::DoesNotExists
            }
        } else {
            PathState::DoesNotExists
        }
    }
}

pub struct CopyState {
    source_state: StatusQuery,
    destination_state: StatusQuery,
    merge: bool,
    overwrite: bool
}

impl CopyState {
    pub fn new(operation: CopyOperation) -> Result<CopyState, OperationError> {
        Ok(
            CopyState {
                source_state: StatusQuery::new(operation.source()),
                destination_state: StatusQuery::new(operation.destination()),
                merge: operation.merge(),
                overwrite: operation.overwrite()
            }
        )
    }

    pub fn atomics() -> MopsTransaction {
        //business stuff
        unimplemented!()
    }
}


pub trait MicroOperation<F> {
    fn apply(fs: &mut F) -> Result<(), OperationError>;
}

pub struct CreateEmptyDirectory {
    path: PathBuf
}

impl MicroOperation<VirtualFileSystem> for CreateEmptyDirectory {
    fn apply(fs: &mut VirtualFileSystem) -> Result<(), OperationError> {
        unimplemented!()
    }
}

