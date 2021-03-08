// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use super::{
    super::{
        Scheduler,
        scheduling::{
            Scheduling,
            MicroOperation
        }
    },
    CreateStrategy,
    CreateOperation
};

impl Scheduler for CreateOperation {
    fn schedule(&self) -> Scheduling {
        use CreateStrategy::*;
        match self.strategy() {
            FileCreation => vec![
                MicroOperation::CreateEmptyFile(self.request().path().to_path_buf())
            ],
            DirectoryCreation => vec![
                MicroOperation::CreateEmptyDirectory(self.request().path().to_path_buf())
            ],
            DirectoryCreationOverwrite => vec![
                MicroOperation::RemoveFile(self.request().path().to_path_buf()),
                MicroOperation::CreateEmptyDirectory(self.request().path().to_path_buf())
            ],
            FileCreationOverwrite => vec![
                MicroOperation::RemoveFile(self.request().path().to_path_buf()),
                MicroOperation::CreateEmptyFile(self.request().path().to_path_buf())
            ]
        }
    }
}