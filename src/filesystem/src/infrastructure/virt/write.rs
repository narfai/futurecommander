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
use std::{ path::{ Path } };

use crate::{
    Kind,
    port::{
        WriteableFileSystem,
        ReadableFileSystem,
        FileSystemAdapter,
        Entry,
        EntryAdapter
    },
    infrastructure::{
        errors::{
            InfrastructureError
        },
        virt::{
            VirtualFileSystem,
            entry_status::{ VirtualStatus },
            representation::{
                VirtualState,
                VirtualDelta,
                VirtualPath
            }
        }
    }
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
                return Err(InfrastructureError::Custom("Delete path does not exists".to_string()))
            ,
        }
        Ok(())
    }

    fn safe_parent(&self, path: &Path) -> Result<EntryAdapter<VirtualStatus>, InfrastructureError> {
        let parent_entry = self.status(VirtualDelta::get_parent_or_root(path).as_path())?;
        if ! parent_entry.exists() {
            Err(InfrastructureError::Custom("Parent does not exists".to_string()))
        } else if !parent_entry.is_dir() {
            Err(InfrastructureError::Custom("Parent is not a directory".to_string()))
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
        let parent = self.safe_parent(dst)?;

        if !source.exists() {
            return Err(InfrastructureError::Custom("Source does not exists".to_string()));
        }

        if destination.exists() && ! destination.is_file() {
            return Err(InfrastructureError::Custom("Destination path must be file".to_string()));
        }

        if source.exists() && ! source.is_file() {
            return Err(InfrastructureError::Custom("Source path must be file".to_string()));
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

        let parent = self.safe_parent(dst)?;

        if !source.exists() {
            return Err(InfrastructureError::Custom("Source does not exists".to_string()));
        }

        if destination.exists() {
            return Err(InfrastructureError::Custom("Destination already exists".to_string()));
        }

        if source.exists() && ! source.is_dir() {
            return Err(InfrastructureError::Custom("Source path must be a directory".to_string()));
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
        self.remove(path)
    }

    fn remove_empty_directory(&mut self, path: &Path) -> Result<(), InfrastructureError>{
        self.remove(path)
    }
}
