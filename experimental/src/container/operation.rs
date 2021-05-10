/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::path::{PathBuf};
use crate::{Result, WriteFileSystem};

#[derive(Debug, Clone)]
pub enum Operation {
    Copy(PathBuf, PathBuf),
    Rename(PathBuf, PathBuf),
    RemoveFile(PathBuf),
    RemoveDir(PathBuf),
    RemoveDirAll(PathBuf),
    CreateDirAll(PathBuf),
    CreateDir(PathBuf),
    CreateFile(PathBuf)
}

impl <F: WriteFileSystem>Operation {
    pub fn apply(&self, fs: &mut F) -> Result<()> {
        use Operation::*;
        match self {
            Copy(from, to) => { fs.copy(&from, &to)?; },
            Rename(from, to) => { fs.rename(&from, &to)?; },
            RemoveFile(path) => { fs.remove_file(&path)?; },
            RemoveDir(path) => { fs.remove_dir(&path)?; },
            RemoveDirAll(path) => { fs.remove_dir_all(&path)?; },
            CreateDir(path) => { fs.create_dir(&path)?; },
            CreateDirAll(path) => { fs.create_dir_all(&path)?; },
            CreateFile(path) => { fs.create_file(&path)?; }
        };
        Ok(())
    }
}