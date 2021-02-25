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
    fmt::Debug,
    collections::HashMap,
    path::PathBuf
};

use serde::{Serialize, Deserialize};

mod copy;
mod create;
mod mov;
mod remove;
mod rnd;

pub mod capability;

pub use self::{
    copy::CopyOperationDefinition,
    create::CreateOperationDefinition,
    mov::MoveOperationDefinition,
    remove::RemoveOperationDefinition
};

use crate::{
    errors::DomainError,
    capability::{
        Guard,
        RegistrarGuard,
        Capabilities
    },
    port::{
        Entry,
        ReadableFileSystem,
        AtomicTransaction
    }
};

pub trait Listener {
    fn emit(&mut self, operation: FileSystemOperation, guard: Box<dyn Guard>) -> Result<(), DomainError>;
}

pub type OperationRegistry = HashMap<PathBuf, Capabilities>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileSystemOperation {
    Create(CreateOperationDefinition, OperationRegistry),
    Copy(CopyOperationDefinition, OperationRegistry),
    Move(MoveOperationDefinition, OperationRegistry),
    Remove(RemoveOperationDefinition, OperationRegistry)
}

impl FileSystemOperation {
    pub fn atomize<E: Entry, F: ReadableFileSystem<Item=E>>(self, fs: &F, guard: Box<dyn Guard>) -> Result<AtomicTransaction, DomainError> {
        match self {
            FileSystemOperation::Create(operation, registry) => create::atomize(operation, fs, &mut RegistrarGuard::new(guard, registry)),
            FileSystemOperation::Copy(operation, registry) => copy::atomize(operation, fs, &mut RegistrarGuard::new(guard, registry)),
            FileSystemOperation::Move(operation, registry) => mov::atomize(operation, fs, &mut RegistrarGuard::new(guard, registry)),
            FileSystemOperation::Remove(operation, registry) => remove::atomize(operation, fs, &mut RegistrarGuard::new(guard, registry)),
        }
    }

    pub fn create(definition: CreateOperationDefinition) -> Self {
        FileSystemOperation::Create(definition, HashMap::new())
    }

    pub fn copy(definition: CopyOperationDefinition) -> Self {
        FileSystemOperation::Copy(definition, HashMap::new())
    }

    pub fn mov(definition: MoveOperationDefinition) -> Self {
        FileSystemOperation::Move(definition, HashMap::new())
    }

    pub fn remove(definition: RemoveOperationDefinition) -> Self {
        FileSystemOperation::Remove(definition, HashMap::new())
    }
}