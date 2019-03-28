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

use std::path::{ PathBuf, Path };
use std::ffi::OsString;
use crate::{ VirtualFileSystem, VfsError, VirtualKind, IdentityStatus, VirtualPath };
use crate::operation::{ WriteOperation, Virtual };

pub struct Copy {
    source: PathBuf,
    destination: PathBuf,
    name: Option<OsString>
}

impl Copy {
    pub fn new(source: &Path, destination: &Path, name: Option<OsString>) -> Copy {
        Copy {
            source: source.to_path_buf(),
            destination: destination.to_path_buf(),
            name
        }
    }
}

impl Virtual<Copy> {
    fn create_new_virtual_identity(source: &VirtualPath, destination: &VirtualPath, with_name: &Option<OsString>) -> Result<VirtualPath, VfsError> {
        if destination.is_contained_by(&source) {
            return Err(VfsError::CopyIntoItSelft(source.to_identity(), destination.to_identity()));
        }

        let mut new_identity = VirtualPath::from(
            source.to_identity(),
            source.to_source(),
            source.to_kind()
        )?.with_new_identity_parent(destination.as_identity());

        if let Some(name) = &with_name {
            new_identity = new_identity.with_file_name(name.as_os_str());
        }

        Ok(new_identity)
    }

    fn copy_virtual_children(mut fs: &mut VirtualFileSystem, source: &VirtualPath, identity: &VirtualPath) -> Result<(), VfsError> {
        match identity.to_kind() {
            VirtualKind::Directory => {
                for child in fs.read_dir(source.as_identity())? {
                    match fs.status(child.as_identity())? {
                        IdentityStatus::ExistsVirtually(_) =>
                            Virtual(Copy::new(
                                child.as_identity(),
                                identity.as_identity(),
                                None
                            )).execute(&mut fs)?,
                        _ => {}
                    };
                }
                Ok(())
            },
            _ => Ok(())
        }
    }
}

impl WriteOperation<VirtualFileSystem> for Virtual<Copy>{
    fn execute(&self, mut fs: &mut VirtualFileSystem) -> Result<(), VfsError> {
        let source = match fs.stat(self.0.source.as_path())? {
            Some(virtual_identity) => virtual_identity,
            None => return Err(VfsError::DoesNotExists(self.0.source.to_path_buf()))
        };

        let destination = match fs.stat(self.0.destination.as_path())? {
            Some(virtual_identity) => match virtual_identity.to_kind() {
                VirtualKind::Directory => virtual_identity,
                _ => return Err(VfsError::IsNotADirectory(self.0.destination.to_path_buf()))
            },
            None => return Err(VfsError::DoesNotExists(self.0.destination.to_path_buf()))
        };

        let new_identity = Self::create_new_virtual_identity(
            &source,
            &destination,
            &self.0.name
        )?;

        if fs.exists(new_identity.as_identity())? {
            return Err(VfsError::AlreadyExists(new_identity.to_identity()));
        }

        fs.mut_add_state().attach_virtual(&new_identity)?;

        if fs.sub_state().get(new_identity.as_identity())?.is_some() {
            fs.mut_sub_state().detach(new_identity.as_identity())?
        }

        Self::copy_virtual_children(&mut fs, &source, &new_identity)
    }

    fn reverse(&self, fs: &mut VirtualFileSystem) -> Result<(), VfsError> {
        Ok(())
    }
}
