// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use serde::{ Serialize, Deserialize };
use crate::{
    WriteableFileSystem,
    InfrastructureError,
};
use super::{
    Request,
    Strategy,
    Scheduler,
    OperationInterface,
    Scheduling,
    OperationGenerator,
    copy::{ CopyRequest, CopyStrategy },
    mov::{ MoveRequest, MoveStrategy },
    create::{ CreateRequest, CreateStrategy },
    remove::{ RemoveRequest, RemoveStrategy },
};


#[derive(Serialize, Deserialize)]
pub struct Operation<S: Strategy, R: Request> {
    strategy: S,
    request: R
}

impl <S: Strategy, R: Request>Operation<S, R> {
    pub fn new(strategy: S, request: R) -> Self {
        Operation { strategy, request }
    }

    pub fn strategy(&self) -> S { self.strategy }

    pub fn request(&self) -> &R { &self.request }
}

impl <S: Strategy, R: Request>OperationInterface for Operation<S, R> where Self: Scheduler + Into<OperationWrapper> {
    fn apply<F: WriteableFileSystem>(&self, fs: &mut F) -> Result<(), InfrastructureError> {
        for atomic_operation in self.schedule() {
            atomic_operation.apply(fs)?
        }
        Ok(())
    }
}

// ========================================= //


#[derive(Serialize, Deserialize, Clone)]
pub enum OperationWrapper {
    Copy { strategy: CopyStrategy, request: CopyRequest },
    Remove { strategy: RemoveStrategy, request: RemoveRequest },
    Move { strategy: MoveStrategy, request: MoveRequest },
    Create { strategy: CreateStrategy, request: CreateRequest }
}

impl OperationWrapper {
    pub fn apply<F: WriteableFileSystem>(self, fs: &mut F) -> Result<(), InfrastructureError>{
        use OperationWrapper::*;
        match self {
            Copy{ strategy, request } => Operation::new(strategy, request).apply(fs),
            Remove{ strategy, request } => Operation::new(strategy, request).apply(fs),
            Move{ strategy, request } => Operation::new(strategy, request).apply(fs),
            Create{ strategy, request } => Operation::new(strategy, request).apply(fs),
        }
    }
}

impl From<Operation<CopyStrategy, CopyRequest>> for OperationWrapper {
    fn from(operation: Operation<CopyStrategy, CopyRequest>) -> OperationWrapper {
        OperationWrapper::Copy{ strategy: operation.strategy(), request: operation.request().clone() }
    }
}

impl From<Operation<CreateStrategy, CreateRequest>> for OperationWrapper {
    fn from(operation: Operation<CreateStrategy, CreateRequest>) -> OperationWrapper {
        OperationWrapper::Create{ strategy: operation.strategy(), request: operation.request().clone() }
    }
}

impl From<Operation<MoveStrategy, MoveRequest>> for OperationWrapper {
    fn from(operation: Operation<MoveStrategy, MoveRequest>) -> OperationWrapper {
        OperationWrapper::Move{ strategy: operation.strategy(), request: operation.request().clone() }
    }
}

impl From<Operation<RemoveStrategy, RemoveRequest>> for OperationWrapper {
    fn from(operation: Operation<RemoveStrategy, RemoveRequest>) -> OperationWrapper {
        OperationWrapper::Remove{ strategy: operation.strategy(), request: operation.request().clone() }
    }
}