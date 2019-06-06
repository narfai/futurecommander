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
    error::Error,
    fmt
};

use bincode::{ Error as BincodeError };

use futurecommander_filesystem::{
    QueryError,
    DomainError
};

#[derive(Debug)]
pub enum DaemonError {
    Io(io::Error),
    Domain(DomainError),
    Query(QueryError),
    ContextKeyDoesNotExists(String),
    ContextValueDoesNotExists(String),
    ContextCannotCast(String, String),
    BinaryEncode(BincodeError),
    InvalidRequest,
    InvalidResponse,
    Exit
}

impl DaemonError {
    pub fn serializable(&self) -> (String, String, String) {
        let kind = match self { //TODO macro for that
            Io => "Io",
            Domain => "Domain",
            Query => "Query",
            ContextKeyDoesNotExists => "ContextKeyDoesNotExists",
            ContextValueDoesNotExists => "ContextValueDoesNotExists",
            ContextCannotCast => "ContextCannotCast",
            BinaryEncode => "BinaryEncode",
            InvalidRequest => "InvalidRequest",
            InvalidResponse => "InvalidResponse",
            Exit => "Exit"
        };

        let message = format!("{}", self);

        (
            kind.to_string(),
            message,
            self.description().to_string()
        )
    }
}

impl From<DomainError> for DaemonError {
    fn from(error: DomainError) -> Self {
        DaemonError::Domain(error)
    }
}

impl From<io::Error> for DaemonError {
    fn from(error: io::Error) -> Self {
        DaemonError::Io(error)
    }
}

impl From<QueryError> for DaemonError {
    fn from(error: QueryError) -> Self {
        DaemonError::Query(error)
    }
}

impl From<BincodeError> for DaemonError {
    fn from(error: BincodeError) -> Self {
        DaemonError::BinaryEncode(error)
    }
}

impl fmt::Display for DaemonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DaemonError::Domain(error) => write!(f, "Domain error {}", error),
            DaemonError::Io(error) => write!(f, "I/O error {}", error),
            DaemonError::Query(error) => write!(f, "Filesystem query error {}", error),
            DaemonError::InvalidRequest => write!(f, "Invalid Request"),
            DaemonError::InvalidResponse => write!(f, "Invalid Response"),
            DaemonError::BinaryEncode(error) => write!(f, "Binary encode error {:?}", error),
            DaemonError::ContextKeyDoesNotExists(key) => write!(f, "Context key {} does not exists", key),
            DaemonError::ContextValueDoesNotExists(key) => write!(f, "Context value for key {} does not exists", key),
            DaemonError::ContextCannotCast(from, to) => write!(f, "Context cannot cast from {} to {}", from, to),
            DaemonError::Exit => write!(f, "Exit"),
        }
    }
}


impl Error for DaemonError {
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            DaemonError::Io(err) => Some(err),
            DaemonError::Query(err) => Some(err),
            DaemonError::BinaryEncode(err) => Some(err),
            DaemonError::Domain(err) => Some(err),
            _ => None
        }
    }
}
