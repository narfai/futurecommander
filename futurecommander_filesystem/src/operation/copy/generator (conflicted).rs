use crate::{
    Entry,
    ReadableFileSystem,
    DomainError,
    operation::{
        OperationGeneratorInterface,
        Strategist,
        copy::{
            CopyOperation,
            CopyGenerator,
            request::CopyRequest,
            strategy::CopyStrategy
        }
    }
};

pub enum CopyGeneratorState<'a, E: Entry + 'a> {
    Uninitialized,
    SelfOperation(CopyStrategy),
    ChildrenOperation {
        children_iterator: Box<dyn Iterator<Item = E> + 'a>,
        opt_operation_generator: Option<Box<CopyGenerator<'a, E>>>
    },
    Terminated
}

impl <E: Entry>Default for CopyGeneratorState<'_, E> {
    fn default() -> Self { CopyGeneratorState::Uninitialized }
}

impl <E: Entry, F: ReadableFileSystem<Item=E>>OperationGeneratorInterface<E, F> for CopyGenerator<'_, E> {
    type Item = CopyOperation;
    fn next(&mut self, fs: &F) -> Result<Option<Self::Item>, DomainError> {
        use CopyGeneratorState::*;
        match self.mut_state() {
            Uninitialized => {
                self.set_state(SelfOperation(self.strategize(fs)?));
                self.next(fs)
            },
            SelfOperation(strategy) => {
                let _strategy = strategy.clone();
                let _request = self.request().clone();

                self.set_state(
                    match _strategy {
                        CopyStrategy::DirectoryMerge => ChildrenOperation {
                            children_iterator: Box::new(fs.read_dir(self.request().source())?.into_iter()),
                            opt_operation_generator: None
                        },
                        CopyStrategy::DirectoryCopy => ChildrenOperation {
                            children_iterator: Box::new(fs.read_maintained(self.request().source())?.into_iter()),
                            opt_operation_generator: None
                        },
                        _ => Terminated
                    }
                );

                Ok(Some(CopyOperation::new(
                    _strategy,
                    _request
                )))
            },
            ChildrenOperation { children_iterator, opt_operation_generator } => {
                if let Some(operation_generator) = opt_operation_generator {
                    if let Some(operation) = operation_generator.next(fs)? {
                        return Ok(Some(operation));
                    }
                }
                if let Some(entry) = children_iterator.next() {
                    let destination = self.mut_request().destination().join(entry.name().unwrap());
                    //TODO #NoUnwrap
                    *opt_operation_generator = Some(Box::new(
                        CopyGenerator::new(
                            CopyRequest::new(
                                entry.to_path(),
                                destination
                            )
                        )
                    ));
                } else {
                    self.set_state(Terminated);
                }
                self.next(fs)
            },
            Terminated => Ok(None)
        }
    }
}
