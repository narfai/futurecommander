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

use std::cmp::Ordering;
use crate::{ VfsError };
use crate::file_system::{ VirtualFileSystem, RealFileSystem };
use crate::representation::{ VirtualPath };
use crate::operation::{WriteOperation, CopyOperation, CreateOperation, RemoveOperation};


//TODO replace by transaction

#[derive(Debug, Clone)]
pub struct ApplyOperation<T>(Vec<T>, Option<usize>);

//impl <T> ApplyOperation<Box<dyn WriteOperation<T>>> {
//    pub fn new() -> Self {
//        ApplyOperation(Vec::<Box<dyn WriteOperation<T>>>::new(), None)
//    }
//
//    fn insert_operation(&mut self, operation: Box<dyn WriteOperation<T>>) {
//        self.0.push(operation);
//    }
//}

//impl ApplyOperation<Box<dyn WriteOperation<RealFileSystem>>> {
//    pub fn from_virtual_filesystem(vfs: &VirtualFileSystem) -> Result<ApplyOperation<Box<dyn WriteOperation<RealFileSystem>>>, VfsError> {
//        unimplemented!()
//        let mut apply = ApplyOperation::<Box<dyn WriteOperation<RealFileSystem>>>::new();
//
//// Cas du renaming d'un item ou de son parent
////        cp A/C Z
////        rm C
////
////        cp B C
////        rm B
////
////        cp Z B
////        rm Z
//
//        for add_identity in vfs.virtual_state()?.iter() {
//            match add_identity.as_source() {
//                Some(source) =>
//                    match source.exists() {
//                        true => {
//                            apply.insert_operation(
//                                Box::new(
//                                    Real::<CopyOperation>::new(
//                                        source,
//                                        VirtualPath::get_parent_or_root(add_identity.as_identity()).as_path(),
//                                        match add_identity.as_identity().file_name() {
//                                            Some(name) => Some(name.to_os_string()),
//                                            None => None
//                                        },
//                                        add_identity.version()
//                                    )
//                                )
//                            );
//                        },
//                        false => return Err(VfsError::SourceDoesNotExists(source.to_path_buf()))
//                    },
//                None => {
//                    apply.insert_operation(
//                        Box::new(
//                            Real::<CreateOperation>::new(
//                                add_identity.as_identity(),
//                                add_identity.to_kind(),
//                                add_identity.version()
//                            )
//                        )
//                    );
//                }//just touch or mkdir
//            }
//        }
//
//        for sub_identity in vfs.reverse_state()?.iter() {
//            match sub_identity.as_source() {
//                Some(source) =>
//                    match source.exists() {
//                        true => {
//                            apply.insert_operation(
//                                Box::new(
//                                    Real::<RemoveOperation>::new(
//                                        source,
//                                        sub_identity.version()
//                                    )
//                                )
//                            );
//                        },
//                        false => return Err(VfsError::SourceDoesNotExists(source.to_path_buf()))
//                    },
//                None => { /*Ignore virtualpath from sub which have no source*/} //just touch or mkdir
//            }
//        }

//        Ok(apply)
//    }
//}
//
//impl <T> WriteOperation<T> for ApplyOperation<Box<dyn WriteOperation<T>>> {
//    fn execute(&self, fs: &mut T) -> Result<(), VfsError> {
//        unimplemented!()
//        self.0.sort_by(
//            |operation_a , operation_b|
//                match operation_a.virtual_version() {
//                    Some(version_a) =>
//                        match operation_b.virtual_version() {
//                            Some(version_b) => version_a.cmp(&version_b).reverse(),
//                            None => Ordering::Equal
//                        },
//                    None => Ordering::Equal
//                }
//        );
//
//
//        for operation in self.0.iter_mut() {
//            if operation.virtual_version().is_none() { //TODO think about how this could be usefull
//                continue
//            }
//            match operation.execute(fs) {
//                Ok(_) => {},
//                Err(error) => panic!("FAILED OPERATION {:#?} {}", operation, error)
//            }
//        }
//        self.1 = Some(RealVersion::increment());//TODO sum of all real fs operation
//        Ok(())
//    }
//}
