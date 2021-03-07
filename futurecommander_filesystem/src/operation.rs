// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

mod scheduling;
mod generator;
mod operation;

mod copy;
mod remove;
mod create;
mod mov;

use std::path::Path;
use serde::{ Serialize };
use crate::{
    DomainError,
    InfrastructureError,
    WriteableFileSystem,
    ReadableFileSystem,
    Entry
};
pub use self::{
    generator::{ OperationGenerator },
    operation::{ Operation },
    scheduling::{ Scheduling, MicroOperation },
    copy::{ CopyRequest, CopyStrategy },
    mov::{ MoveRequest, MoveStrategy },
    create::{ CreateRequest, CreateStrategy },
    remove::{ RemoveRequest, RemoveStrategy },
};


pub trait Strategy: Copy + Serialize {}
pub trait Request: Clone + Serialize {
    fn target(&self) -> &Path;
}

pub trait Scheduler {
    fn schedule(&self) -> scheduling::Scheduling;
}

pub trait Strategist {
    type Strategy: Copy;
    fn strategize<F: ReadableFileSystem>(&self, fs: &F) -> Result<Self::Strategy, DomainError>;
}

pub trait OperationInterface: Scheduler + Serialize {
    fn apply<F: WriteableFileSystem>(&self, fs: &mut F) -> Result<(), InfrastructureError>;
}

pub trait OperationGeneratorInterface<E: Entry, F: ReadableFileSystem<Item=E>>: Strategist {
    type Item: OperationInterface;
    fn next(&mut self, fs: &F) -> Result<Option<Self::Item>, DomainError>;
}