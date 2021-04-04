/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::path::Path;

use crate::{
    Result,
    ReadFileSystem,
    Metadata,
    ReadDir
};

use super::Container;

impl ReadFileSystem for Container {
    fn metadata<P: AsRef<Path>>(&self, path: P) -> Result<Metadata> {
        self.preview.metadata(path)
    }

    fn read_dir<P: AsRef<Path>>(&self, path: P) -> Result<ReadDir> {
        self.preview.read_dir(path)
    }
}