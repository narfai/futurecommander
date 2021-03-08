// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use serde::{ Serialize, Deserialize };
use crate::{ Kind };

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub enum SerializableKind {
    File,
    Directory,
    Unknown
}

impl From<Kind> for SerializableKind {
    fn from(kind: Kind) -> Self {
        match kind {
            Kind::File => SerializableKind::File,
            Kind::Directory => SerializableKind::Directory,
            Kind::Unknown => SerializableKind::Unknown,
        }
    }
}

impl From<SerializableKind> for Kind {
    fn from(kind: SerializableKind) -> Self {
        match kind {
            SerializableKind::File => Kind::File,
            SerializableKind::Directory => Kind::Directory,
            SerializableKind::Unknown => Kind::Unknown,
        }
    }
}