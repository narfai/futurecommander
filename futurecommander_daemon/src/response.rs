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

use serde::{ Serialize, Deserialize };

use bincode::{ deserialize, serialize };

use crate::{
    errors::{ DaemonError },
    SerializableEntry,
};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum ResponseKind {
    Collection,
    Entry
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum ResponseStatus {
    Success,
    Fail
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub id: String,
    pub kind: ResponseKind,
    pub status: ResponseStatus,
    pub content: Option<Vec<SerializableEntry>>,
    pub error: Option<String>
}

impl Response {
    pub fn decode(payload: &[u8]) -> Result<Self, DaemonError> {
        Ok(deserialize(payload)?)
    }

    pub fn encode(self) -> Result<Vec<u8>, DaemonError> {
        Ok(serialize(&self)?)
    }
}
