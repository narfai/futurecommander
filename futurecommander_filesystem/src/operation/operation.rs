use crate::{
    port::WriteableFileSystem,
    infrastructure::errors::InfrastructureError,
    operation::{
        Request,
        Scheduler,
        OperationInterface
    }
};

pub struct Operation<S: Copy, R: Request> {
    strategy: S,
    request: R
}

impl <S: Copy, R: Request>Operation<S, R> {
    pub fn new(strategy: S, request: R) -> Self {
        Operation { strategy, request }
    }

    pub fn strategy(&self) -> S { self.strategy }

    pub fn request(&self) -> &R { &self.request }
}

impl <S: Copy, R: Request>OperationInterface for Operation<S, R> where Self: Scheduler {
    fn apply<F: WriteableFileSystem>(&self, fs: &mut F) -> Result<(), InfrastructureError> {
        for atomic_operation in self.schedule() {
            atomic_operation.apply(fs)?
        }
        Ok(())
    }
}

