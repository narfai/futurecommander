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
    path::{ PathBuf, Path },
    ffi::{ OsString, OsStr }
};

use serde::{ Serialize, Deserialize };

use crate::{
    port::{
        Entry
    }
};

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableEntry {
    pub path: PathBuf,
    pub name: Option<String>,
    pub is_dir: bool,
    pub is_file: bool,
    pub exists: bool
}

impl SerializableEntry {
    pub fn from(entry: &Entry) -> Self {
        SerializableEntry {
            path: entry.to_path(),
            name: if let Some(s) = entry.name() {
                Some(s.to_string_lossy().to_string())
            } else { None },
            is_dir: entry.is_dir(),
            is_file: entry.is_file(),
            exists: entry.exists()
        }
    }
}


