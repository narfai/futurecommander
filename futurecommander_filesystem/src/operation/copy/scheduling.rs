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
    CopyStrategy,
    CopyOperation
};

impl Scheduler for CopyOperation {
    fn schedule(&self) -> Scheduling {
        use CopyStrategy::*;
        match self.strategy() {
            FileCopy => vec![
                MicroOperation::CopyFileToFile{
                    source: self.request().source().to_path_buf(),
                    destination: self.request().destination().to_path_buf()
                }
            ],
            FileOverwrite => vec![
                MicroOperation::RemoveFile(self.request().destination().to_path_buf()),
                MicroOperation::CopyFileToFile {
                    source: self.request().source().to_path_buf(),
                    destination: self.request().destination().to_path_buf()
                }
            ],
            DirectoryCopy => vec![
                MicroOperation::BindDirectoryToDirectory {
                    source: self.request().source().to_path_buf(),
                    destination: self.request().destination().to_path_buf()
                }
            ],
            _ => Vec::new()
        }
    }
}