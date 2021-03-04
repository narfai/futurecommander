mod scheduling;
mod generator;
mod operation;

mod copy;

use crate::{
    errors::DomainError,
    infrastructure::errors::InfrastructureError,
    port::{
        ReadableFileSystem,
        WriteableFileSystem,
        Entry
    }
};

pub use self::{
    generator::{ OperationGenerator },
    copy::{ CopyRequest }
};

pub trait Request: Clone {}

pub trait Scheduler {
    fn schedule(&self) -> scheduling::Scheduling;
}

pub trait Strategist {
    type Strategy: Copy;
    fn strategize<F: ReadableFileSystem>(&self, fs: &F) -> Result<Self::Strategy, DomainError>;
}

pub trait OperationInterface: Scheduler {
    fn apply<F: WriteableFileSystem>(&self, fs: &mut F) -> Result<(), InfrastructureError>;
}

pub trait OperationGeneratorInterface<E: Entry, F: ReadableFileSystem<Item=E>>: Strategist {
    type Item: OperationInterface;
    fn next(&mut self, fs: &F) -> Result<Option<Self::Item>, DomainError>;
}