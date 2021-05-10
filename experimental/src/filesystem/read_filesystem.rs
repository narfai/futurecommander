/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::path::Path;

use crate::{,
    ReadFileSystem,
    Result
};

use super::{
    FileSystem,
    ReadDir,
    Metadata,
    MetadataExt
};

impl ReadFileSystem for FileSystem {
    /// Errors :
    /// * The user lacks permissions to perform `metadata` call on `path`.
    /// * `path` does not exist.
    fn metadata<P: AsRef<Path>>(&self, path: P) -> Result<Metadata> {
        path.as_ref().metadata().into_virtual_metadata()
    }

    /// Errors :
    /// * The provided `path` doesn't exist.
    /// * The process lacks permissions to view the contents.
    /// * The `path` points at a non-directory file.
    fn read_dir<P: AsRef<Path>>(&self, path: P) -> Result<ReadDir> {
        Ok(ReadDir::new(path.as_ref(), Vec::new()))
    }
}