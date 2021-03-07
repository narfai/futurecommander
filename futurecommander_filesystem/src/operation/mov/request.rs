// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use serde::{ Serialize, Deserialize };
use std::path::{ PathBuf, Path };
use super::super::{ Request };

#[derive(Clone, Serialize, Deserialize)]
pub struct MoveRequest {
    source: PathBuf,
    destination: PathBuf
}

impl Request for MoveRequest {
    fn target(&self) -> &Path { &self.destination }
}

impl MoveRequest {
    pub fn new(source: PathBuf, destination: PathBuf) -> Self {
        MoveRequest { source, destination }
    }

    pub fn source(&self) -> &Path { &self.source }

    pub fn destination(&self) -> &Path { &self.destination }
}