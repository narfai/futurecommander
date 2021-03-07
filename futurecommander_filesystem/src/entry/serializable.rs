// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use serde::{ Serialize, Deserialize };
use super::{ Entry };

#[derive(Serialize, PartialEq, Deserialize, Debug, Clone)]
pub struct SerializableEntry {
    pub name: Option<String>,
    pub is_dir: bool,
    pub is_file: bool,
    pub is_virtual: bool
}

impl Eq for SerializableEntry {}

impl SerializableEntry {
    pub fn from(entry: &dyn Entry) -> Self {
        SerializableEntry {
            name: if let Some(s) = entry.name() {
                Some(s.to_string_lossy().to_string())
            } else { None },
            is_dir: entry.is_dir(),
            is_file: entry.is_file(),
            is_virtual: entry.is_virtual()
        }
    }
}


