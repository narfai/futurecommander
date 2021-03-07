// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2019-2021 François CADEILLAN

use crate::{
    Entry,
    ReadableFileSystem,
    DomainError,
};
use super::{
    super::{
        OperationGeneratorInterface,
        Strategist,
    },
    CopyOperation,
    CopyGenerator,
    request::CopyRequest,
    strategy::CopyStrategy
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

impl <E: Entry>CopyGenerator<'_, E> {
    pub fn new(request: CopyRequest) -> Self {
        CopyGenerator {
            request,
            state: CopyGeneratorState::Uninitialized
        }
    }
}

impl <E: Entry, F: ReadableFileSystem<Item=E>>OperationGeneratorInterface<E, F> for CopyGenerator<'_, E> {
    type Item = CopyOperation;
    fn next(&mut self, fs: &F) -> Result<Option<Self::Item>, DomainError> {
        use CopyGeneratorState::*;
        match &mut self.state {
            Uninitialized => {
                let strategy = self.strategize(fs)?;
                self.state = SelfOperation(strategy);
                self.next(fs)
            },
            SelfOperation(strategy) => {
                let _strategy = strategy.clone();
                let _request = self.request.clone();

                self.state = match _strategy {
                    CopyStrategy::DirectoryMerge => ChildrenOperation {
                        children_iterator: Box::new(fs.read_dir(self.request.source())?.into_iter()),
                        opt_operation_generator: None
                    },
                    CopyStrategy::DirectoryCopy => ChildrenOperation {
                        children_iterator: Box::new(fs.read_maintained(self.request.source())?.into_iter()),
                        opt_operation_generator: None
                    },
                    _ => Terminated
                };

                Ok(Some(CopyOperation::new(
                    _strategy,
                    _request
                )))
            },
            ChildrenOperation { children_iterator, opt_operation_generator }=> {
                if let Some(operation_generator) = opt_operation_generator {
                    if let Some(operation) = operation_generator.next(fs)? {
                        return Ok(Some(operation));
                    }
                }
                if let Some(entry) = children_iterator.next() {
                    *opt_operation_generator = Some(Box::new(
                        CopyGenerator::new(
                            CopyRequest::new(
                                entry.to_path(),
                                //TODO #NoUnwrap
                                self.request.destination().join(entry.name().unwrap()).to_path_buf()
                            )
                        )
                    ));
                } else {
                    self.state = Terminated;
                }
                self.next(fs)
            },
            Terminated => Ok(None)
        }
    }
}
