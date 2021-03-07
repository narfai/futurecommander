// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use std::{ path::{ PathBuf }};
use crate::{
    InfrastructureError,
    WriteableFileSystem
};

#[derive(Debug)]
pub enum MicroOperation {
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

impl MicroOperation {
    pub fn apply<F: WriteableFileSystem>(&self, fs: &mut F) -> Result<(), InfrastructureError> {
        use self::MicroOperation::*;
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

pub type Scheduling = Vec<MicroOperation>;