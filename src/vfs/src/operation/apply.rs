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


#[derive(Debug)]
pub struct ApplyOperation<T>(Vec<T>, Option<usize>);

impl <T> ApplyOperation<Box<dyn WriteOperation<T>>> {
    pub fn new() -> Self {
        ApplyOperation(Vec::<Box<WriteOperation<T>>>::new(), None)
    }

    fn insert_operation(&mut self, operation: Box<WriteOperation<T>>) {
        self.0.push(operation);
    }
}

impl ApplyOperation<Box<WriteOperation<RealFileSystem>>> {
    pub fn from_virtual_filesystem(vfs: &VirtualFileSystem) -> Result<ApplyOperation<Box<dyn WriteOperation<RealFileSystem>>>, VfsError> {
        let mut apply = ApplyOperation::<Box<dyn WriteOperation<RealFileSystem>>>::new();

        for add_identity in vfs.virtual_state()?.iter() {
            match add_identity.as_source() {
                Some(source) =>
                    match source.exists() {
                        true => {
                            apply.insert_operation(
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
                None => {
                    apply.insert_operation(
                        Box::new(
                            Real::<CreateOperation>::new(
                                add_identity.as_identity(),
                                add_identity.to_kind(),
                                Some(add_identity.version())
                            )
                        )
                    );
                }//just touch or mkdir
            }
        }

        for sub_identity in vfs.reverse_state()?.iter() {
            match sub_identity.as_source() {
                Some(source) =>
                    match source.exists() {
                        true => {
                            apply.insert_operation(
                                Box::new(
                                    Real::<RemoveOperation>::new(
                                        source,
                                        Some(sub_identity.version())
                                    )
                                )
                            );
                        },
                        false => return Err(VfsError::SourceDoesNotExists(source.to_path_buf()))
                    },
                None => { /*Ignore virtualpath from sub which have no source*/} //just touch or mkdir
            }
        }

        for op in apply.0.iter() {
            println!("{:?}", op);
        }

        Ok(apply)
    }
}

impl <T> WriteOperation<T> for ApplyOperation<Box<dyn WriteOperation<T>>> {
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
            if operation.virtual_version().is_none() {
                continue
            }
            println!("{:?}", operation);
            operation.execute(fs)?
        }
        self.1 = Some(RealVersion::increment());//TODO sum of all real fs operation
        Ok(())
    }

    fn virtual_version(&self) -> Option<usize> { None }
    fn real_version(&self) -> Option<usize> { self.1 }
    fn debug(&self) -> String {
        format!("{} {}", "ApplyOperation".to_string(), self.0.len())
    }
}
