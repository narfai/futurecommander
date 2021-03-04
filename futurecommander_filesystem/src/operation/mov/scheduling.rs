use crate::{
    operation::{
        Scheduler,
        scheduling::{
            Scheduling,
            MicroOperation
        },
        mov::{
            MoveStrategy,
            MoveOperation
        }
    }
};

impl Scheduler for MoveOperation {
    fn schedule(&self) -> Scheduling {
        use MoveStrategy::*;
        match self.strategy() {
            FileMove => vec![
                MicroOperation::MoveFileToFile {
                    source: self.request().source().to_path_buf(),
                    destination: self.request().destination().to_path_buf()
                }
            ],
            FileOverwrite => vec![
                MicroOperation::RemoveFile(self.request().destination().to_path_buf()),
                MicroOperation::MoveFileToFile {
                    source: self.request().source().to_path_buf(),
                    destination: self.request().destination().to_path_buf()
                }
            ],
            DirectoryMoveBefore => vec![
                MicroOperation::BindDirectoryToDirectory {
                    source: self.request().source().to_path_buf(),
                    destination: self.request().destination().to_path_buf()
                }
            ],
            DirectoryMoveAfter => vec![
                MicroOperation::RemoveMaintainedEmptyDirectory(self.request().source().to_path_buf())
            ],
            DirectoryMerge => vec![
                MicroOperation::RemoveEmptyDirectory(self.request().source().to_path_buf())
            ]
        }
    }
}