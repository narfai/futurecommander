use crate::{
    operation::{ Request }
};

pub struct OperationGenerator<S: Default, R: Request> {
    pub request: R,
    pub state: S
}

impl <S: Default, R: Request>OperationGenerator<S, R> {
    pub fn new(request: R) -> Self {
        OperationGenerator {
            request,
            state: S::default()
        }
    }
}