// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN
use std::{ path::{ Path } };

use futurecommander_representation::{
    VirtualState,
    VirtualDelta,
    VirtualPath
};

use crate::{
    Kind,
    Entry,
    EntryAdapter
};
use super::super::{
    InfrastructureError,
    ReadableFileSystem,
    WriteableFileSystem,
    FileSystemAdapter,
    VirtualFileSystem,
    VirtualStatus
};

impl FileSystemAdapter<VirtualFileSystem> where Self : ReadableFileSystem {
    fn remove(&mut self, path: &Path) -> Result<(), InfrastructureError> {
        match self.status(path)?.into_inner() {
            VirtualStatus{ state: VirtualState::Exists, identity }
            | VirtualStatus{ state: VirtualState::Replaced, identity }
            | VirtualStatus{ state: VirtualState::ExistsThroughVirtualParent, identity } => {
                self.0.mut_sub_state().attach_virtual(&identity)?;
            },
            VirtualStatus{ state: VirtualState::ExistsVirtually, identity } => {
                self.0.mut_add_state().detach(identity.as_identity())?;
                if let Some(source) = identity.as_source() {
                    if let VirtualStatus{
                        state: VirtualState::Replaced,
                        identity: virtual_path
                    } = self.status(source)?.into_inner() {
                        if self.0.add_state().is_directory_empty(virtual_path.as_identity()) {
                            self.0.mut_add_state().detach(virtual_path.as_identity())?;
                        }
                    }
                }
            }
            VirtualStatus{ state: VirtualState::NotExists, .. }
            | VirtualStatus{ state: VirtualState::Removed, .. }
            | VirtualStatus{ state: VirtualState::RemovedVirtually, .. } =>
                return Err(InfrastructureError::PathDoesNotExists(path.to_path_buf()))
            ,
        }
        Ok(())
    }

    fn safe_parent(&self, path: &Path) -> Result<EntryAdapter<VirtualStatus>, InfrastructureError> {
        let parent_entry = self.status(VirtualDelta::get_parent_or_root(path).as_path())?;
        if ! parent_entry.exists() {
            Err(InfrastructureError::ParentDoesNotExists(parent_entry.to_path()))
        } else if !parent_entry.is_dir() {
            Err(InfrastructureError::ParentIsNotADirectory(parent_entry.to_path()))
        } else {
            Ok(parent_entry)
        }
    }

    fn create(&mut self, new_identity: VirtualPath, destination: EntryAdapter<VirtualStatus>) -> Result<(), InfrastructureError>{
        let state = destination.as_inner().state();
        match destination.into_inner() {
            VirtualStatus{ state: VirtualState::Exists, identity }
            | VirtualStatus{ state: VirtualState::ExistsVirtually, identity }
            | VirtualStatus{ state: VirtualState::ExistsThroughVirtualParent, identity}
            | VirtualStatus{ state: VirtualState::Replaced, identity } => {
                match state {
                    VirtualState::Replaced
                    | VirtualState::ExistsVirtually => {
                        self.0.mut_add_state().detach(identity.as_identity())?;
                    },
                    _ => {}
                }

                self.0.mut_add_state().attach_virtual( & new_identity)?;
            },
            VirtualStatus{ state: VirtualState::NotExists, .. } => {
                self.0.mut_add_state().attach_virtual(&new_identity)?;
            },
            VirtualStatus{ state: VirtualState::Removed, .. }
            | VirtualStatus{ state: VirtualState::RemovedVirtually, .. } => {
                self.0.mut_sub_state().detach(new_identity.as_identity())?;
                self.0.mut_add_state().attach_virtual(&new_identity)?;
            },
        };
        Ok(())
    }
}

impl WriteableFileSystem for FileSystemAdapter<VirtualFileSystem> {
    //Write virtual specialization
    fn create_empty_directory(&mut self, path: &Path) -> Result<(), InfrastructureError> {
        self.safe_parent(path)?;
        let existing = self.status(path)?;
        self.create(
            VirtualPath::from(
                path.to_path_buf(),
                None,
                Kind::Directory
            )?,
            existing
        )
    }

    fn create_empty_file(&mut self, path: &Path) -> Result<(), InfrastructureError> {
        self.safe_parent(path)?;
        let existing = self.status(path)?;
        self.create(
            VirtualPath::from(
                path.to_path_buf(),
                None,
                Kind::File
            )?,
            existing
        )
    }

    fn copy_file_to_file(&mut self, src: &Path, dst: &Path) -> Result<(), InfrastructureError>{
        let source = self.status(src)?;
        let destination = self.status(dst)?;
        self.safe_parent(dst)?;

        if !source.exists() {
            return Err(InfrastructureError::SourceDoesNotExists(source.to_path()));
        }

        if destination.exists() && ! destination.is_file() {
            return Err(InfrastructureError::DestinationIsNotAFile(destination.to_path()));
        }

        if source.exists() && ! source.is_file() {
            return Err(InfrastructureError::SourceIsNotAFile(source.to_path()));
        }

        let source_identity = source.as_inner().as_virtual();

        self.create(
            VirtualPath::from(
                destination.to_path(),
                source_identity.to_source(),
                source_identity.to_kind()
            )?,
            destination
        )
    }

    fn move_file_to_file(&mut self, source: &Path, destination: &Path) -> Result<(), InfrastructureError>{
        self.copy_file_to_file(source, destination)?;
        self.remove(source)?;
        Ok(())
    }

    fn bind_directory_to_directory(&mut self, src: &Path, dst: &Path) -> Result<(), InfrastructureError> {
        let source = self.status(src)?;
        let destination = self.status(dst)?;

        self.safe_parent(dst)?;

        if !source.exists() {
            return Err(InfrastructureError::SourceDoesNotExists(source.to_path()));
        }

        if destination.exists() {
            return Err(InfrastructureError::DestinationAlreadyExists(destination.to_path()));
        }

        if source.exists() && ! source.is_dir() {
            return Err(InfrastructureError::SourceIsNotADirectory(source.to_path()));
        }

        let source_identity = source.as_inner().as_virtual();

        self.create(
            VirtualPath::from(
                destination.to_path(),
                source_identity.to_source(),
                source_identity.to_kind()
            )?,
            destination
        )
    }


    fn remove_file(&mut self, path: &Path) -> Result<(), InfrastructureError> {
        if ! self.status(path)?.exists() {
            return Err(InfrastructureError::PathDoesNotExists(path.to_path_buf()));
        }

        self.remove(path)
    }

    fn remove_empty_directory(&mut self, path: &Path) -> Result<(), InfrastructureError>{
        if ! self.status(path)?.exists() {
            return Err(InfrastructureError::PathDoesNotExists(path.to_path_buf()));
        }

        if ! self.read_dir(path)?.is_empty() {
            return Err(InfrastructureError::DirectoryIsNotEmpty(path.to_path_buf()));
        }

        self.remove(path)
    }

    fn remove_maintained_empty_directory(&mut self, path: &Path) -> Result<(), InfrastructureError>  {
        if ! self.status(path)?.exists() {
            return Err(InfrastructureError::PathDoesNotExists(path.to_path_buf()));
        }

        if ! self.read_maintained(path)?.is_empty() {
            return Err(InfrastructureError::DirectoryIsNotEmpty(path.to_path_buf()));
        }

        self.remove(path)
    }

}
