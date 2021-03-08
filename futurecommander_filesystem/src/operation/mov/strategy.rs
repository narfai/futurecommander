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
    MoveGenerator
};

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum MoveStrategy {
    DirectoryMerge,
    FileOverwrite,
    FileMove,
    DirectoryMoveBefore,
    DirectoryMoveAfter,
}

impl Strategy for MoveStrategy {}

impl From<MoveStrategy> for Option<Capability> {
    fn from(strategy: MoveStrategy) -> Self {
        use MoveStrategy::*;
        match strategy {
            DirectoryMerge => Some(Capability::Merge),
            FileOverwrite => Some(Capability::Overwrite),
            _ => None
        }
    }
}

impl <E: Entry>Strategist for MoveGenerator<'_, E> {
    type Strategy = MoveStrategy;
    fn strategize<F: ReadableFileSystem>(&self, fs: &F) -> Result<Self::Strategy, DomainError> {
        use MoveStrategy::*;
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
            Ok(DirectoryMoveBefore)
        } else if source.is_file() {
            Ok(FileMove)
        } else {
            Err(DomainError::Custom(String::from("Unknown node source type")))
        }
    }
}