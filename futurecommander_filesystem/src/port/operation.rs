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

//Generalize OperationScheduler<S>, OperationGenerator, OperationGeneratorState<S, D>