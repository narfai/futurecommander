/*
 * Copyright 2019 François CADEILLAN
 *
 * This file is part of FutureCommander.
 *
 * FutureCommander is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * FutureCommander is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with FutureCommander.  If not, see <https://www.gnu.org/licenses/>.
 */


use std::{
    io,
    error,
    fmt
};

use crate::{
    virt::errors::VirtualError,
    real::errors::RealError,
    representation::errors::RepresentationError
};

#[derive(Debug)]
pub enum OperationError {
    Io(io::Error),
    Virtual(VirtualError),
    Real(RealError),
    Representation(RepresentationError)
}

impl From<io::Error> for OperationError {
    fn from(error: io::Error) -> Self {
        OperationError::Io(error)
    }
}

impl From<VirtualError> for OperationError {
    fn from(error: VirtualError) -> Self {
        OperationError::Virtual(error)
    }
}

impl From<RealError> for OperationError {
    fn from(error: RealError) -> Self {
        OperationError::Real(error)
    }
}

impl From<RepresentationError> for OperationError {
    fn from(error: RepresentationError) -> Self {
        OperationError::Representation(error)
    }
}


impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperationError::Virtual(error) => write!(f, "Virtual error {}", error),
            OperationError::Io(error) => write!(f, "Io error {}", error),
            OperationError::Real(error) => write!(f, "Real error {}", error),
            OperationError::Representation(error) => write!(f, "Representation error {}", error),
        }
    }
}


impl error::Error for OperationError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            OperationError::Io(err) => Some(err),
            OperationError::Virtual(err) => Some(err),
            OperationError::Real(err) => Some(err),
            OperationError::Representation(err) => Some(err),
            _ => None
        }
    }
}
