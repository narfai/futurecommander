use std::iter;
use crate::{
    errors::{ DomainError },
    infrastructure::errors::{ InfrastructureError },
    port::{
        WriteableFileSystem,
        ReadableFileSystem,
        Entry
    }
};

pub trait Operation {
    fn apply<F: WriteableFileSystem>(&self, fs: &mut F) -> Result<(), InfrastructureError> ;
}

pub trait OperationGenerator<E: Entry> {
    type Item: Operation;
    fn next<F: ReadableFileSystem<Item=E>>(&mut self, fs: &F) -> Result<Option<Self::Item>, DomainError>;
}

pub struct OperationGeneratorAdapter<'a, E: Entry + 'a, T> {
    pub inner: T,
    pub sent_itself: bool,
    pub children_iterator: Box<dyn Iterator<Item = E> + 'a>,
    pub operation_generator: Option<Box<OperationGeneratorAdapter<'a, E, T>>>
}

impl <'a, E: Entry + 'a, T>OperationGeneratorAdapter<'a, E, T> {
    pub fn new(inner: T) -> Self {
        OperationGeneratorAdapter {
            inner,
            sent_itself: false,
            children_iterator: Box::new(iter::empty()),
            operation_generator: None,
        }
    }
}
