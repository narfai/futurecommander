// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

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