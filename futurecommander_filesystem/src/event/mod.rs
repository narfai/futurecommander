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
    fmt::Debug
};

use serde::{Serialize, Deserialize};

mod copy;
mod create;
mod mov;
mod remove;

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
        RegistrarGuard
    },
    port::{
        Entry,
        ReadableFileSystem,
        AtomicTransaction
    }
};

pub trait Listener {
    fn emit(&mut self, operation: &FileSystemOperation, guard: RegistrarGuard) -> Result<RegistrarGuard, DomainError>;
}

pub trait Delayer {
    fn delay(&mut self, operation: FileSystemOperation, guard: RegistrarGuard);
}

pub trait Previewer {
    fn preview<G: Guard>(&mut self, operation: FileSystemOperation, guard: &G);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileSystemOperation {
    Create(CreateOperationDefinition),
    Copy(CopyOperationDefinition),
    Move(MoveOperationDefinition),
    Remove(RemoveOperationDefinition)
}

impl FileSystemOperation {
    pub fn atomize<E: Entry, F: ReadableFileSystem<Item=E>>(&self, fs: &F, guard: &mut dyn Guard) -> Result<AtomicTransaction, DomainError> {
        match self {
            FileSystemOperation::Create(operation) => create::atomize(operation, fs, guard),
            FileSystemOperation::Copy(operation) => copy::atomize(operation, fs, guard),
            FileSystemOperation::Move(operation) => mov::atomize(operation, fs, guard),
            FileSystemOperation::Remove(operation) => remove::atomize(operation, fs, guard),
        }
    }

    pub fn create(definition: CreateOperationDefinition) -> Self {
        FileSystemOperation::Create(definition)
    }

    pub fn copy(definition: CopyOperationDefinition) -> Self {
        FileSystemOperation::Copy(definition)
    }

    pub fn mov(definition: MoveOperationDefinition) -> Self {
        FileSystemOperation::Move(definition)
    }

    pub fn remove(definition: RemoveOperationDefinition) -> Self {
        FileSystemOperation::Remove(definition)
    }
}