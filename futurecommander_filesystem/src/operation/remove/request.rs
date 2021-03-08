// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use serde::{ Serialize, Deserialize };
use std::path::{ PathBuf, Path };
use super::super::{ Request };

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RemoveRequest {
    path: PathBuf
}

impl Request for RemoveRequest {
    fn target(&self) -> &Path { &self.path }
}

impl RemoveRequest {
    pub fn new(path: PathBuf) -> Self {
        RemoveRequest { path }
    }

    pub fn path(&self) -> &Path { &self.path }
}