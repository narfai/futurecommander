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

use serde::{
    Serialize,
    Deserialize
};

use futurecommander_filesystem::{
    SerializableEntry
};

use bincode::{ serialize };
use crate::{
    errors::{
        DaemonError
    },
    response::{
        Response,
        ResponseAdapter,
        SerializableResponse
    }
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EntriesResponse(pub Option<Vec<SerializableEntry>>);

#[typetag::serde]
impl SerializableResponse for ResponseAdapter<EntriesResponse> {
    fn serializable(&self) -> Box<SerializableResponse> {
        Box::new(self.clone())
    }
}

impl Response for ResponseAdapter<EntriesResponse> {
    fn encode(&self) -> Result<Vec<u8>, DaemonError> {
        let mut binary_response = vec![self.header as u8];
        binary_response.append(&mut serialize(self)?);
        Ok(binary_response)
    }
}
