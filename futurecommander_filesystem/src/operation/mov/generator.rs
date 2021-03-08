// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use crate::{
    Entry,
    ReadableFileSystem,
    DomainError
};
use super::{
    super::{
        OperationGeneratorInterface,
        Strategist,
    },
    MoveOperation,
    MoveGenerator,
    request::MoveRequest,
    strategy::MoveStrategy
};

pub enum MoveGeneratorState<'a, E: Entry + 'a> {
    Uninitialized,
    SelfOperation(MoveStrategy),
    ChildrenOperation {
        strategy: MoveStrategy,
        children_iterator: Box<dyn Iterator<Item = E> + 'a>,
        opt_operation_generator: Option<Box<MoveGenerator<'a, E>>>
    },
    Terminated
}

impl <'a, E: Entry> Default for MoveGeneratorState<'a, E> {
    fn default() -> Self {
        Self::Uninitialized
    }
}

impl <E: Entry, F: ReadableFileSystem<Item=E>>OperationGeneratorInterface<E, F> for MoveGenerator<'_, E> {
    type Item = MoveOperation;
    fn next(&mut self, fs: &F) -> Result<Option<Self::Item>, DomainError> {
        use MoveGeneratorState::*;
        match &mut self.state {
            Uninitialized => {
                let strategy = self.strategize(fs)?;
                match strategy {
                    MoveStrategy::DirectoryMerge => {
                        self.state = ChildrenOperation {
                            strategy,
                            children_iterator: Box::new(fs.read_dir(&self.request.source())?.into_iter()),
                            opt_operation_generator: None
                        }
                    },
                    _ => {
                        self.state = SelfOperation(strategy)
                    }
                }

                self.next(fs)
            },
            SelfOperation(strategy) => {
                let _strategy = strategy.clone();
                self.state = match _strategy {
                    MoveStrategy::DirectoryMoveBefore => ChildrenOperation {
                        strategy: _strategy,
                        children_iterator: Box::new(fs.read_maintained(&self.request.source())?.into_iter()),
                        opt_operation_generator: None
                    },
                    _ => Terminated
                };

                Ok(Some(MoveOperation::new(_strategy, self.request.clone())))
            },
            ChildrenOperation { strategy, children_iterator, opt_operation_generator }=> {
                if let Some(operation_generator) = opt_operation_generator {
                    if let Some(operation) = operation_generator.next(fs)? {
                        return Ok(Some(operation));
                    }
                }
                if let Some(entry) = children_iterator.next() {
                    *opt_operation_generator = Some(Box::new(
                        MoveGenerator::new(
                            MoveRequest::new(
                                entry.to_path(),
                                //TODO #NoUnwrap
                                self.request.destination().join(entry.name().unwrap()).to_path_buf()
                            )
                        )
                    ));
                } else {
                    self.state = match strategy {
                        MoveStrategy::DirectoryMerge => SelfOperation(MoveStrategy::DirectoryMerge),
                        MoveStrategy::DirectoryMoveBefore => SelfOperation(MoveStrategy::DirectoryMoveAfter),
                        _ => Terminated
                    }
                }
                self.next(fs)
            },
            Terminated => Ok(None)
        }
    }
}
