// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use super::{ Request };

pub struct OperationGenerator<S, R: Request> {
    pub(in crate::operation) request: R,
    pub(in crate::operation) state: S
}