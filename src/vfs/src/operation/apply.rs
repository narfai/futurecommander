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

use std::slice::Iter;
use std::cmp::Ordering;
use crate::{ Real, VfsError };
use crate::file_system::{ VirtualFileSystem, RealFileSystem, RealVersion };
use crate::representation::{ VirtualKind, VirtualPath };
use crate::operation::{WriteOperation, CopyOperation, CreateOperation, RemoveOperation};
use crate::query::{ReadQuery, StatusQuery};


pub struct ApplyOperation<T>(Vec<T>, Option<usize>);

impl <T> ApplyOperation<Box<WriteOperation<T>>> {
    pub fn new() -> Self {
        ApplyOperation(Vec::<Box<WriteOperation<T>>>::new(), None)
    }

    fn add_operation(&mut self, operation: Box<WriteOperation<T>>) {
        self.0.push(operation);
    }
}

impl ApplyOperation<Box<WriteOperation<RealFileSystem>>> {
    fn from_virtual_filesystem(vfs: &VirtualFileSystem) -> Result<Self, VfsError> {
        let mut apply = Self::new();

        for add_identity in vfs.virtual_state()?.iter() {
            match add_identity.as_source() {
                Some(source) =>
                    match source.exists() {
                        true => {
                            apply.add_operation(
                                Box::new(
                                    Real::<CopyOperation>::new(
                                        source,
                                        VirtualPath::get_parent_or_root(add_identity.as_identity()).as_path(),
                                        match add_identity.as_identity().file_name() {
                                            Some(name) => Some(name.to_os_string()),
                                            None => None
                                        },
                                        Some(add_identity.version())
                                    )
                                )
                            );
                        },
                        false => return Err(VfsError::SourceDoesNotExists(source.to_path_buf()))
                    },
                None =>
                    match add_identity.to_kind() {
                        VirtualKind::File | VirtualKind::Directory => {
                            apply.add_operation(
                                Box::new(
                                    Real::<CreateOperation>::new(
                                        add_identity.as_identity(),
                                        add_identity.to_kind(),
                                        Some(add_identity.version())
                                    )
                                )
                            );
                        },
                        VirtualKind::Unknown => {/*unknown unblocking error*/} //TODO @symlink
                    }//just touch or mkdir
            }
        }

        for reverse in vfs.reverse_state()?.iter() {

        }

        Ok(apply)
    }
}

impl <T> WriteOperation<T> for ApplyOperation<Box<WriteOperation<T>>> {
    fn execute(&mut self, fs: &mut T) -> Result<(), VfsError> {
        self.0.sort_by(
            |operation_a , operation_b|
                match operation_a.virtual_version() {
                    Some(version_a) =>
                        match operation_b.virtual_version() {
                            Some(version_b) => version_a.cmp(&version_b),
                            None => Ordering::Equal
                        },
                    None => Ordering::Equal
                }
        );

        for operation in self.0.iter_mut() {
            operation.execute(fs)?
        }
        self.1 = Some(RealVersion::increment());//TODO sum of all real fs operation
        Ok(())
    }

    fn virtual_version(&self) -> Option<usize> {
        self.1
    }
    fn real_version(&self) -> Option<usize> { self.1 }
}
