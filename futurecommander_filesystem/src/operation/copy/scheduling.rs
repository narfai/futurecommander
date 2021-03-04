use crate::{
    operation::{
        Scheduler,
        scheduling::{
            Scheduling,
            MicroOperation
        },
        copy::{
            CopyStrategy,
            CopyOperation
        }
    }
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