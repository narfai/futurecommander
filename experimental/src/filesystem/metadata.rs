/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::fs::Metadata as FsMetadata;

use crate::Result;

use super::{
    FileType,
    FileTypeExt,
    MetadataExt
};

#[derive(Debug, Clone)]
pub struct Metadata {
    pub (in crate) file_type: FileType
}

impl Metadata {
    pub fn new(file_type: FileType) -> Self {
        Metadata { file_type }
    }
    pub fn file_type(&self) -> FileType { self.file_type }
    pub fn is_dir(&self) -> bool { self.file_type().is_dir() }
    pub fn is_file(&self) -> bool { self.file_type().is_file() }
    // pub fn len(&self) -> u64 { unimplemented!(); }
    // pub fn permissions(&self) -> Permissions { unimplemented!; }
    // pub fn modified(&self) -> io::Result<SystemTime> { unimplemented!; }
    // pub fn accessed(&self) -> io::Result<SystemTime> { unimplemented!; }
    // pub fn created(&self) -> io::Result<SystemTime> { unimplemented!; }
}

impl MetadataExt for FsMetadata {
    fn into_virtual_metadata(self) -> Result<Metadata> {
        Ok(
            Metadata {
                file_type: self.file_type().into_virtual_file_type()?
            }
        )
    }
}

//TODO test different preview of real