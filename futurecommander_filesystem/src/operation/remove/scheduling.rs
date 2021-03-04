use crate::{
    operation::{
        Scheduler,
        scheduling::{
            Scheduling,
            MicroOperation
        },
        remove::{
            RemoveStrategy,
            RemoveOperation
        }
    }
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
            ],
            _ => Vec::new()       }
    }
}