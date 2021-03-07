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
    OperationInterface
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

impl <S: Strategy, R: Request>OperationInterface for Operation<S, R> where Self: Scheduler {
    fn apply<F: WriteableFileSystem>(&self, fs: &mut F) -> Result<(), InfrastructureError> {
        for atomic_operation in self.schedule() {
            atomic_operation.apply(fs)?
        }
        Ok(())
    }
}