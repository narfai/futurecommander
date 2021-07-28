/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::fs::FileType as FsFileType;

use crate::{
    FileSystemError,
    preview::NodeFileType,
    Result
};

use super::FileTypeExt;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum FileType {
    File,
    Directory,
    Symlink
}

impl FileType {
    pub fn is_dir(&self) -> bool { matches!(self, FileType::Directory) }
    pub fn is_file(&self) -> bool { matches!(self, FileType::File) }
    pub fn is_symlink(&self) -> bool { matches!(self, FileType::Symlink) }
}

impl FileTypeExt for FsFileType {
    fn into_virtual_file_type(self) -> Result<FileType> {
        if self.is_symlink() {
            Ok(FileType::Symlink)
        } else if self.is_dir() {
            Ok(FileType::Directory)
        } else if self.is_file() {
            Ok(FileType::File)
        } else {
            Err(FileSystemError::UnknownFileType)
        }
    }
}


//TODO test different preview of real