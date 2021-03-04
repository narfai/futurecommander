use std::path::PathBuf;

use crate::{
    Kind,
    Entry,
    ReadableFileSystem,
    DomainError,
    operation::{
        OperationGeneratorInterface,
        Strategist,
        create::{
            CreateOperation,
            CreateGenerator,
            request::CreateRequest,
            strategy::CreateStrategy
        }
    }
};

pub enum CreateGeneratorState {
    Uninitialized,
    ParentOperation(Vec<PathBuf>),
    SelfOperation,
    Terminated
}

impl Default for CreateGeneratorState {
    fn default() -> Self { CreateGeneratorState::Uninitialized }
}

impl <E: Entry, F: ReadableFileSystem<Item=E>>OperationGeneratorInterface<E, F> for CreateGenerator {
    type Item = CreateOperation;
    fn next(&mut self, fs: &F) -> Result<Option<Self::Item>, DomainError> {
        use CreateGeneratorState::*;
        match &mut self.state {
            Uninitialized => {
                let mut ancestors = self.request.path().ancestors();
                ancestors.next();
                self.state = ParentOperation(
                    ancestors.map(|path| path.to_path_buf()).collect()
                );
                self.next(fs)
            },
            ParentOperation(ancestors)=> {
                if let Some(path) = ancestors.pop(){
                    let entry = fs.status(&path)?;
                    if !entry.exists() {
                        return Ok(
                            Some(
                                CreateOperation::new(
                                    CreateStrategy::DirectoryCreation,
                                    CreateRequest::new(entry.to_path(), Kind::Directory)
                                )
                            )
                        );
                    }
                } else {
                    self.state = SelfOperation
                }
                self.next(fs)
            },
            SelfOperation => {
                self.state = Terminated;
                Ok(
                    Some(
                        CreateOperation::new(
                            self.strategize(fs)?,
                            self.request.clone()
                        )
                    )
                )
            },
            Terminated => Ok(None)
        }
    }
}
