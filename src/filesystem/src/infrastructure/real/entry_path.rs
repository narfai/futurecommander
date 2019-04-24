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

use std::{
    path::{ Path, PathBuf },
    ffi::{ OsStr }
};

use crate::{
    port::{
        Entry,
        EntryAdapter
    }
};

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

impl Entry for EntryAdapter<PathBuf> {
    fn path(&self) -> &Path { self.0.as_path() }

    fn to_path(&self) -> PathBuf { self.0.clone() }

    fn name(&self) -> Option<&OsStr> { self.0.file_name() }

    fn is_dir(&self) -> bool { self.0.is_dir() }

    fn is_file(&self) -> bool { self.0.is_file() }

    fn exists(&self) -> bool { self.0.exists() }
}
