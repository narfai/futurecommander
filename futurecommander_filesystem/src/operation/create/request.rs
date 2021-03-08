// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use serde::{ Serialize, Deserialize };
use std::path::{ PathBuf, Path };
use crate::Kind;
use super::{
    super::Request,
    serializable_kind::SerializableKind
};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreateRequest {
    path: PathBuf,
    kind: SerializableKind
}

impl Request for CreateRequest {
    fn target(&self) -> &Path { &self.path }
}

impl CreateRequest {
    pub fn new(path: PathBuf, kind: Kind) -> Self {
        CreateRequest { path, kind: kind.into() }
    }

    pub fn path(&self) -> &Path { &self.path }

    pub fn kind(&self) -> Kind { self.kind.into() }
}