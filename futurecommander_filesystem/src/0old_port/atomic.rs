// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::{
    path    ::{ PathBuf },
    vec     ::{ IntoIter }
};

use crate::{
    port    ::{
        WriteableFileSystem
    },
    infrastructure::{ errors:: InfrastructureError }

};

//TODO move to operation and rename AtomicOperation
#[derive(Debug)]
pub enum Atomic {
    CreateEmptyDirectory(PathBuf),
    CreateEmptyFile(PathBuf),
    BindDirectoryToDirectory {
        source: PathBuf,
        destination: PathBuf
    },
    CopyFileToFile {
        source: PathBuf,
        destination: PathBuf
    },
    MoveFileToFile {
        source: PathBuf,
        destination: PathBuf
    },
    RemoveFile(PathBuf),
    RemoveEmptyDirectory(PathBuf),
    RemoveMaintainedEmptyDirectory(PathBuf)
}

impl Atomic {
    pub fn apply<F: WriteableFileSystem>(&self, fs: &mut F) -> Result<(), InfrastructureError> {
        use self::Atomic::*;
        match self {
            CreateEmptyDirectory(path) => fs.create_empty_directory(path.as_path()),
            CreateEmptyFile(path) => fs.create_empty_file(path.as_path()),
            BindDirectoryToDirectory { source, destination } => fs.bind_directory_to_directory(source.as_path(), destination.as_path()),
            CopyFileToFile { source, destination } => fs.copy_file_to_file(source.as_path(), destination.as_path()),
            MoveFileToFile { source, destination } => fs.move_file_to_file(source.as_path(), destination.as_path()),
            RemoveFile(path) => fs.remove_file(path.as_path()),
            RemoveEmptyDirectory(path) => fs.remove_empty_directory(path.as_path()),
            RemoveMaintainedEmptyDirectory(path) => fs.remove_maintained_empty_directory(path.as_path())
        }
    }
}

// TODO TO DELETE
#[derive(Default, Debug)]
pub struct AtomicTransaction(pub Vec<Atomic>);

impl AtomicTransaction {
    pub fn new(atomics: Vec<Atomic>) -> Self{
        AtomicTransaction(atomics)
    }

    pub fn add(&mut self, atomic: Atomic) {
        self.0.push(atomic)
    }

    pub fn apply<F: WriteableFileSystem>(self, fs: &mut F) -> Result<(), InfrastructureError> {
        for atomic in self.0 {
            atomic.apply(fs)?
        }
        Ok(())
    }

    pub fn merge(&mut self, transaction: AtomicTransaction) {
        for atomic in transaction {
            self.add(atomic);
        }
    }
}

impl IntoIterator for AtomicTransaction {
    type Item = Atomic;
    type IntoIter = IntoIter<Atomic>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

