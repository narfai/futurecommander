// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use super::{
    super::{
        Scheduler,
        Scheduling,
        MicroOperation
    },
    RemoveStrategy,
    RemoveOperation
};

impl Scheduler for RemoveOperation {
    fn schedule(&self) -> Scheduling {
        use RemoveStrategy::*;
        match self.strategy() {
            FileRemoval => vec![
                MicroOperation::RemoveFile(self.request().path().to_path_buf())
            ],
            EmptyDirectoryRemoval|RecursiveDirectoryRemoval => vec![
                MicroOperation::RemoveEmptyDirectory(self.request().path().to_path_buf())
            ]
        }
    }
}