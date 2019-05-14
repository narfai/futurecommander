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
    path::{ Path }
};

use serde::{ Serialize, Deserialize};

use bincode::{ deserialize, serialize };

use futurecommander_filesystem::{
    Container,
    ReadableFileSystem,
    tools::normalize,
    SerializableEntry
};

use crate::{
    errors::DaemonError,
    Request,
    Response,
    ResponseStatus,
    ResponseKind,
    RequestHeader
};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListRequest {
    pub id: String,
    pub r#type: String,
    pub path: String
}

impl ListRequest {
    pub fn new(id: &str, path: &str) -> ListRequest {
        ListRequest {
            id: id.to_string(),
            r#type: "LIST".to_string(),
            path: path.to_string()
        }
    }
}

impl Request for ListRequest {
    fn header() -> RequestHeader {
        RequestHeader::List
    }

    fn process(&self, container: &mut Container) -> Result<Vec<u8>, DaemonError> {
        let response = match container.read_dir(
            normalize(
                Path::new(&self.path)
            ).as_path()
        ){
            Ok(collection) =>
                (Response {
                    id: self.id.clone(),
                    kind: ResponseKind::Collection,
                    status: ResponseStatus::Success,
                    content: Some(
                        collection
                        .into_iter()
                        .map(|entry| SerializableEntry::from(&entry))
                        .collect::<Vec<SerializableEntry>>()
                    ),
                    error: None
                }).encode()?
            ,
            Err(error) =>
                (Response {
                    id: self.id.clone(),
                    kind: ResponseKind::Collection,
                    status: ResponseStatus::Fail,
                    content: None,
                    error: Some(format!("{}", DaemonError::from(error)))
                }).encode()?
        };
        Ok(response)
    }

    fn as_bytes(&self) -> Result<Vec<u8>, DaemonError> {
        let mut request = vec![Self::header() as u8];
        request.append(&mut serialize(self)?);
        Ok(request)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, DaemonError> where Self: Sized {
        Ok(deserialize(bytes)?)
    }
}
