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
    error::Error,
    fmt,
    io
};

use bincode::{ Error as BincodeError };

use futurecommander_filesystem::{ QueryError, DomainError };

#[derive(Debug)]
pub enum ProtocolError {
    Io(io::Error),
    Domain(DomainError),
    Query(QueryError),
    BinaryEncode(BincodeError),
    MessageParsing,
    InvalidHeader,
    Exit
}

impl From<DomainError> for ProtocolError {
    fn from(error: DomainError) -> Self {
        ProtocolError::Domain(error)
    }
}

impl From<io::Error> for ProtocolError {
    fn from(error: io::Error) -> Self {
        ProtocolError::Io(error)
    }
}

impl From<QueryError> for ProtocolError {
    fn from(error: QueryError) -> Self {
        ProtocolError::Query(error)
    }
}

impl From<BincodeError> for ProtocolError {
    fn from(error: BincodeError) -> Self {
        ProtocolError::BinaryEncode(error)
    }
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtocolError::Io(error) => write!(f, "Io error {}", error),
            ProtocolError::Domain(error) => write!(f, "Domain error {}", error),
            ProtocolError::Query(error) => write!(f, "Filesystem query error {}", error),
            ProtocolError::InvalidHeader => write!(f, "Invalid Header"),
            ProtocolError::MessageParsing => write!(f, "Cannot parse message"),
            ProtocolError::BinaryEncode(error) => write!(f, "Binary encode error {:?}", error),
            ProtocolError::Exit => write!(f, "Exit"),
        }
    }
}


impl Error for ProtocolError {
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            ProtocolError::Io(err) => Some(err),
            ProtocolError::Query(err) => Some(err),
            ProtocolError::BinaryEncode(err) => Some(err),
            ProtocolError::Domain(err) => Some(err),
            _ => None
        }
    }
}
