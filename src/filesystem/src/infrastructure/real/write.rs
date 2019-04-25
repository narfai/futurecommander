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

use std::{
    path::{ Path, PathBuf },
};

use crate::{
    errors::DomainError,
    port::{
        WriteableFileSystem,
        FileSystemAdapter
    },
    infrastructure::{
        errors::{ InfrastructureError },
        real::{
            RealFileSystem
        }
    }
};

impl WriteableFileSystem for FileSystemAdapter<RealFileSystem> {
    //Write real specialization
    fn create_empty_directory(&mut self, path: &Path) -> Result<(), InfrastructureError> { unimplemented!() }
    fn create_empty_file(&mut self, path: &Path) -> Result<(), InfrastructureError> { unimplemented!() }
    fn copy_file_to_file(&mut self, source: &Path, destination: &Path) -> Result<(), InfrastructureError>{ unimplemented!() }
    fn move_file_to_file(&mut self, source: &Path, destination: &Path) -> Result<(), InfrastructureError>{ unimplemented!() }
    fn remove_file(&mut self, path: &Path) -> Result<(), InfrastructureError> { unimplemented!() }
    fn remove_empty_directory(&mut self, path: &Path) -> Result<(), InfrastructureError>{ unimplemented!() }
}
