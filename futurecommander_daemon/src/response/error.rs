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
    fmt::{
        Debug
    },
    error::{
        Error
    }
};

use serde::{
    Serialize,
    Deserialize
};

use bincode::{ serialize };

use crate::{
    errors::{ DaemonError },
    response::{
        Response,
        SerializableResponse,
        ResponseAdapter
    }
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ErrorResponse{
    kind: String,
    message: String,
    description: String
}


#[typetag::serde]
impl SerializableResponse for ResponseAdapter<ErrorResponse> {
    fn serializable(&self) -> Box<SerializableResponse> {
        Box::new(self.clone())
    }
}

impl From<&DaemonError> for ErrorResponse {
    fn from(error: &DaemonError) -> ErrorResponse {
        let (kind, message, description) = error.serializable();
        ErrorResponse {
            kind,
            message,
            description
        }
    }
}


impl Response for ResponseAdapter<ErrorResponse> {
    fn encode(&self) -> Result<Vec<u8>, DaemonError> {
        let mut binary_response = vec![self.header as u8];
        binary_response.append(&mut serialize(self)?);
        Ok(binary_response)
    }
}
