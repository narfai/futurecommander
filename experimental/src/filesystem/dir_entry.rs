/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::{
    path::{ PathBuf, Path },
    ffi::OsString
};

use crate::Result;

use super::{
    Metadata,
    FileType
};

#[derive(Debug)]
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

