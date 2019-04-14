/*
 * Copyright 2019 François CADEILLAN
 *
 * This file is part of FutureCommander.
 *
 * FutureCommander is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * FutureCommander is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with FutureCommander.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::cmp::Ordering;

use std::path::{ Path, PathBuf };
use std::ffi::{ OsStr };

pub trait Entry {
    fn path(&self) -> &Path;
    fn to_path(&self) -> PathBuf;
    fn name(&self) -> Option<&OsStr>;
    fn is_dir(&self) -> bool;
    fn is_file(&self) -> bool;
    fn exists(&self) -> bool;
}

impl Eq for Entry {}

impl Ord for Entry {
    fn cmp(&self, other: &Entry) -> Ordering {
        self.path().cmp(other.path())
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.path().cmp(other.path()))
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Entry) -> bool {
        self.path().eq(other.path())
    }
}

#[derive(Debug)]
pub struct EntryAdapter<T>(pub T);

impl <T> EntryAdapter<T> {
    pub fn into_inner(self) -> T {
        self.0
    }

    pub fn as_inner(&self) -> &T {
        &self.0
    }
}

impl Entry for EntryAdapter<&Path> {
    fn path(&self) -> &Path {
        self.0
    }

    fn to_path(&self) -> PathBuf {
        self.0.to_path_buf()
    }

    fn name(&self) -> Option<&OsStr> {
        self.0.file_name()
    }

    fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    fn is_file(&self) -> bool {
        self.0.is_file()
    }

    fn exists(&self) -> bool { self.0.exists() }
}

