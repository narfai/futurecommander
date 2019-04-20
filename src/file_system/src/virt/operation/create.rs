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
use std::path::Ancestors;

use crate::{
    VirtualFileSystem,
    Kind,
    OperationError,
    operation::{ Operation, CreateOperation },
    query::{ Query, StatusQuery, Entry },
    virt::{
        errors::VirtualError
    }
};

impl Operation<VirtualFileSystem> for CreateOperation{
    fn execute(&self, fs: &mut VirtualFileSystem) -> Result<(), OperationError> {
        let path = self.path();
        match StatusQuery::new(path).retrieve(&fs)?.into_inner().into_existing_virtual() {
            Some(identity) => {
                fs.mut_add_state().attach_virtual(&identity.with_source(None))?;
            },
            None => {
                if ! self.recursive() || self.kind() == Kind::File {
                    let parent = crate::path_helper::get_parent_or_root(path);
                    let parent_status = StatusQuery::new(parent.as_path()).retrieve(&fs)?;
                    if ! parent_status.exists() {
                        return Err(OperationError::from(VirtualError::ParentDoesNotExists(path.to_path_buf())));
                    } else if !parent_status.is_dir() {
                        return Err(OperationError::from(VirtualError::ParentIsNotADirectory(path.to_path_buf())));
                    }
                    fs.mut_add_state().attach(path, None, self.kind())?;
                } else {
                    fn recursive_dir_creation(fs: &mut VirtualFileSystem, mut ancestors: &mut Ancestors<'_>) -> Result<(), VirtualError> {
                        if let Some(path) = ancestors.next() {
                            if ! StatusQuery::new(path).retrieve(&fs)?.exists() {
                                recursive_dir_creation(fs, &mut ancestors)?;
                                fs.mut_add_state().attach(path, None, Kind::Directory)?;
                            }
                        }
                        Ok(())
                    }
                    recursive_dir_creation(fs, &mut path.ancestors())?;
                }
            }
        }
        Ok(())
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        Samples
    };


    #[test]
    fn virtual_create_operation_directory(){
        let chroot = Samples::init_simple_chroot("virtual_create_operation_directory");
        let mut fs = VirtualFileSystem::default();

        let operation = CreateOperation::new(
            chroot.join("CREATED").as_path(),
            Kind::Directory,
            false,
            false
        );

        operation.execute(&mut fs).unwrap();

        assert!(fs.virtual_state().unwrap().is_virtual(chroot.join("CREATED").as_path()).unwrap());
        assert!(fs.virtual_state().unwrap().is_directory(chroot.join("CREATED").as_path()).unwrap());
    }


    #[test]
    fn virtual_create_operation_directory_recursive(){
        let chroot = Samples::init_simple_chroot("virtual_create_operation_directory_recursive");
        let mut fs = VirtualFileSystem::default();

        let operation = CreateOperation::new(
            chroot.join("CREATED/NESTED/DIRECTORY").as_path(),
            Kind::Directory,
            true,
            false
        );

        operation.execute(&mut fs).unwrap();

        assert!(fs.virtual_state().unwrap().is_virtual(chroot.join("CREATED/NESTED/DIRECTORY").as_path()).unwrap());
        assert!(fs.virtual_state().unwrap().is_directory(chroot.join("CREATED/NESTED/DIRECTORY").as_path()).unwrap());
    }

    #[test]
    fn virtual_create_operation_file(){
        let chroot = Samples::init_simple_chroot("virtual_create_operation_file");
        let mut fs = VirtualFileSystem::default();

        let operation = CreateOperation::new(
            chroot.join("CREATED").as_path(),
            Kind::File,
            false,
            false
        );

        operation.execute(&mut fs).unwrap();

        assert!(fs.virtual_state().unwrap().is_virtual(chroot.join("CREATED").as_path()).unwrap());
        assert!(fs.virtual_state().unwrap().is_file(chroot.join("CREATED").as_path()).unwrap());
    }

    #[test]
    fn virtual_create_operation_file_overwrite(){
        let chroot = Samples::init_simple_chroot("virtual_create_operation_file_overwrite");
        let mut fs = VirtualFileSystem::default();

        let operation = CreateOperation::new(
            chroot.join("RDIR/RFILEA").as_path(),
            Kind::File,
            false,
            true
        );

        operation.execute(&mut fs).unwrap();

        assert!(fs.virtual_state().unwrap().is_virtual(chroot.join("RDIR/RFILEA").as_path()).unwrap());
        assert!(fs.virtual_state().unwrap().is_file(chroot.join("RDIR/RFILEA").as_path()).unwrap());
    }
}
