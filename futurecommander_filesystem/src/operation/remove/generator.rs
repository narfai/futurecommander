use crate::{
    Entry,
    ReadableFileSystem,
    DomainError,
    operation::{
        OperationGeneratorInterface,
        Strategist,
        remove::{
            RemoveOperation,
            RemoveGenerator,
            request::RemoveRequest,
            strategy::RemoveStrategy
        }
    }
};

pub enum RemoveGeneratorState<'a, E: Entry + 'a> {
    Uninitialized,
    ChildrenOperation {
        strategy: RemoveStrategy,
        children_iterator: Box<dyn Iterator<Item = E> + 'a>,
        opt_operation_generator: Option<Box<RemoveGenerator<'a, E>>>
    },
    SelfOperation(RemoveStrategy),
    Terminated
}

impl <E: Entry>Default for RemoveGeneratorState<'_, E> {
    fn default() -> Self { RemoveGeneratorState::Uninitialized }
}

impl <E: Entry, F: ReadableFileSystem<Item=E>>OperationGeneratorInterface<E, F> for RemoveGenerator<'_, E> {
    type Item = RemoveOperation;
    fn next(&mut self, fs: &F) -> Result<Option<Self::Item>, DomainError> {
        use RemoveGeneratorState::*;
        match &mut self.state {
            Uninitialized => {
                let strategy = self.strategize(fs)?;
                match strategy {
                    RemoveStrategy::RecursiveDirectoryRemoval => {
                        self.state = ChildrenOperation{
                            strategy,
                            children_iterator: Box::new(fs.read_dir(&self.request.path())?.into_iter()),
                            opt_operation_generator: None
                        };
                    }
                    _ => { self.state = SelfOperation(strategy); }
                }
                self.next(fs)
            },
            ChildrenOperation{ strategy, children_iterator, opt_operation_generator } => {
                if let Some(operation_generator) = opt_operation_generator {
                    if let Some(operation) = operation_generator.next(fs)? {
                        return Ok(Some(operation));
                    }
                }
                if let Some(entry) = children_iterator.next() {
                    *opt_operation_generator = Some(Box::new(
                        RemoveGenerator::new(
                            RemoveRequest::new(entry.to_path())
                        )
                    ));
                } else {
                    self.state = SelfOperation(*strategy);
                }
                self.next(fs)
            },
            SelfOperation(scheduling) => {
                let _scheduling = scheduling.clone();
                self.state = Terminated;
                Ok(Some(RemoveOperation::new(_scheduling, self.request.clone())))
            },
            Terminated => Ok(None)
        }
    }
}
