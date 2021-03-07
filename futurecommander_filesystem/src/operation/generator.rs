// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use crate::{
    operation::{ Request }
};

pub struct OperationGenerator<S: Default, R: Request> {
    pub(in crate::operation) request: R,
    pub(in crate::operation) state: S
}

impl <S: Default, R: Request>OperationGenerator<S, R> {
    pub fn new(request: R) -> Self {
        OperationGenerator {
            request,
            state: S::default()
        }
    }
}