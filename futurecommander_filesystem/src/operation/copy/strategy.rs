// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use serde::{ Serialize, Deserialize };
use crate::{
    Capability,
    Entry,
    ReadableFileSystem,
    DomainError
};
use super::{
    super::{
        Strategy,
        Strategist
    },
    CopyGenerator
};


#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum CopyStrategy {
    DirectoryMerge,
    FileOverwrite,
    FileCopy,
    DirectoryCopy
}

impl Strategy for CopyStrategy {}

impl From<CopyStrategy> for Option<Capability> {
    fn from(strategy: CopyStrategy) -> Self {
        use CopyStrategy::*;
        match strategy {
            DirectoryMerge => Some(Capability::Merge),
            FileOverwrite => Some(Capability::Overwrite),
            _ => None
        }
    }
}

impl <E: Entry>Strategist for CopyGenerator<'_, E> {
    type Strategy = CopyStrategy;
    fn strategize<F: ReadableFileSystem>(&self, fs: &F) -> Result<Self::Strategy, DomainError> {
        use CopyStrategy::*;
        let source = fs.status(self.request.source())?;
        if !source.exists() {
            return Err(DomainError::SourceDoesNotExists(self.request.source().to_path_buf()))
        }

        let destination = fs.status(self.request.destination())?;
        if source.is_dir() && destination.is_contained_by(&source) {
            return Err(DomainError::CopyIntoItSelf(source.to_path(), destination.to_path()));
        }

        if destination.exists() {
            if source.is_dir() {
                if destination.is_dir() {
                    Ok(DirectoryMerge)
                } else {
                    Err(DomainError::MergeFileWithDirectory(source.to_path(), destination.to_path()))
                }
            } else if source.is_file() {
                if destination.is_file() {
                    Ok(FileOverwrite)
                } else {
                    Err(DomainError::OverwriteDirectoryWithFile(source.to_path(), destination.to_path()))
                }
            } else {
                Err(DomainError::Custom(String::from("Unknown node source type")))
            }
        } else if source.is_dir() {
            Ok(DirectoryCopy)
        } else if source.is_file() {
            Ok(FileCopy)
        } else {
            Err(DomainError::Custom(String::from("Unknown node source type")))
        }
    }
}