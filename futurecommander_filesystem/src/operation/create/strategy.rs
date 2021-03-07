// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use serde::{ Serialize, Deserialize };
use crate::{
    Capability,
    Kind,
    Entry,
    ReadableFileSystem,
    DomainError
};
use super::{
    super::{
        Strategy,
        Strategist,
    },
    CreateGenerator
};

#[derive(Copy, Serialize, Deserialize, Clone, Debug)]
pub enum CreateStrategy {
    FileCreation,
    DirectoryCreation,
    FileCreationOverwrite,
    DirectoryCreationOverwrite,
}

impl Strategy for CreateStrategy {}

impl From<CreateStrategy> for Option<Capability> {
    fn from(strategy: CreateStrategy) -> Self {
        use CreateStrategy::*;
        match strategy {
            FileCreationOverwrite => Some(Capability::Overwrite),
            DirectoryCreationOverwrite => Some(Capability::Overwrite),
            _ => None
        }
    }
}

impl Strategist for CreateGenerator {
    type Strategy = CreateStrategy;
    fn strategize<F: ReadableFileSystem>(&self, fs: &F) -> Result<Self::Strategy, DomainError> {
        use CreateStrategy::*;
        let entry = fs.status(self.request.path())?;

        if entry.exists() {
            if entry.is_dir() {
                return Err(DomainError::DirectoryOverwriteNotAllowed(entry.to_path()))
            } else if !entry.is_file() {
                return Err(DomainError::CreateUnknown(entry.to_path()))
            }
        }

        match self.request.kind().into() {
            Kind::Directory => {
                if entry.exists() && entry.is_file() {
                    Ok(DirectoryCreationOverwrite)
                } else {
                    Ok(DirectoryCreation)
                }
            },
            Kind::File => {
                if entry.exists() && entry.is_file() {
                    Ok(FileCreationOverwrite)
                } else {
                    Ok(FileCreation)
                }
            },
            Kind::Unknown => {
                Err(DomainError::CreateUnknown(entry.to_path()))
            }
        }
    }
}