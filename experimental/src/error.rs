/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::{
    fmt,
    error,
    io,
    path::{ PathBuf }
};

use crate::{Node};

#[derive(Debug)]
pub enum FileSystemError {
    IoError(io::Error),
    Custom(String),
    PathTerminatesInTwoDot(PathBuf),
    PathTerminatesInARootOrPrefix(PathBuf),
    UnknownFileType,
    PathDoesNotExists(PathBuf),
    FromDoesNotExists(PathBuf),
    PathIsNotADirectory(PathBuf),
    DirectoryIsNotEmpty(PathBuf),
    FromAndToAreNotOnTheSameFileSystem(PathBuf, PathBuf),
    ToCannotBeADirectory(PathBuf),
    ToDirectoryIsNotEmpty(PathBuf),
    PathAlreadyExists(PathBuf),
    ParentDoesNotExists(PathBuf),
    FromIsNotAFile(PathBuf),
    ParentIsNotADirectory(PathBuf),
    PathIsADirectory(PathBuf),
    NodeAlreadyExists(Node),
    ParentNodeIsNotADirectory(Node),
    ConvertDeletedNodeToFileType,
    ToParentDoesNotExists(PathBuf),
    ToParentIsNotADirectory(PathBuf)
}

impl From<io::Error> for FileSystemError {
    fn from(error: io::Error) -> Self {
        FileSystemError::IoError(error)
    }
}


impl fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use FileSystemError::*;
        match self {
            IoError(error) => write!(f, "I/O error {}", error),
            Custom(s) => write!(f, "Custom error {}", s),
            PathTerminatesInTwoDot(path) => write!(f, "Path terminates with two dots : {}", path.display()),
            PathTerminatesInARootOrPrefix(path) => write!(f, "Path terminates in a root or prefix {}", path.display()),
            UnknownFileType => write!(f, "Unknown file type"),
            PathDoesNotExists(path) => write!(f, "Path does not exists {}", path.display()),
            FromDoesNotExists(path) => write!(f, "From does not exists {}", path.display()),
            PathIsNotADirectory(path) => write!(f, "Path is not a directory {}", path.display()),
            DirectoryIsNotEmpty(path) => write!(f, "Directory is not empty {}", path.display()),
            FromAndToAreNotOnTheSameFileSystem(from, to) => write!(f, "{} and {} are not on the same filesystem", from.display(), to.display()),
            ToCannotBeADirectory(path) => write!(f, "To cannot be a directory {}", path.display()),
            ToDirectoryIsNotEmpty(path) => write!(f, "To directory is not empty {}", path.display()),
            PathAlreadyExists(path) => write!(f, "Path already exists {}", path.display()),
            ParentDoesNotExists(path) => write!(f, "Parent does not exists {}", path.display()),
            FromIsNotAFile(path) => write!(f, "From is not a file {}", path.display()),
            ParentIsNotADirectory(path) => write!(f, "Parent is not a directory {}", path.display()),
            PathIsADirectory(path) => write!(f, "Path is a directory {}", path.display()),
            NodeAlreadyExists(node) => write!(f, "Node already exists {}", node.name().to_string_lossy()),
            ParentNodeIsNotADirectory(node) => write!(f, "Parent node is not a directory {}", node.name().to_string_lossy()),
            ConvertDeletedNodeToFileType => write!(f, "Convert deleted node to file type"),
            ToParentDoesNotExists(path) => write!(f, "To parent does not exists {}", path.display()),
            ToParentIsNotADirectory(path) => write!(f, "To parent is not a directory {}", path.display()),
        }
    }
}

impl error::Error for FileSystemError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            FileSystemError::IoError(err) => Some(err),
            _ => None
        }
    }
}
