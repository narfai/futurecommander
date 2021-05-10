/*
 * SPDX-License-Identifier: GPL-3.0-only
 * Copyright (C) 2019-2021 François CADEILLAN
 */

use std::{
    path::{ Path },
    fs::{ create_dir, create_dir_all, copy, rename, remove_dir, remove_file, remove_dir_all, File }
};

use crate::{
    Result,
    WriteFileSystem
};

use super::FileSystem;

impl WriteFileSystem for FileSystem {
    fn create_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        File::create(path.as_ref())?;
        Ok(())
    }

    fn create_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        Ok(create_dir(path.as_ref())?)
    }

    fn create_dir_all<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        Ok(create_dir_all(path.as_ref())?)
    }

    fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> Result<u64> {
        Ok(copy(from.as_ref(), to.as_ref())?)
    }

    fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&mut self, from: P, to: Q) -> Result<()> {
        Ok(rename(from.as_ref(), to.as_ref())?)
    }

    fn remove_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        Ok(remove_dir(path.as_ref())?)
    }

    fn remove_dir_all<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        Ok(remove_dir_all(path.as_ref())?)
    }

    fn remove_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        Ok(remove_dir(path.as_ref())?)
    }
}