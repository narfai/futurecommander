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
    Context,
    RequestAdapter

};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListAction {
    pub id: String,
    pub path: String
}

impl ListAction {
    pub fn adapter(context: Context) -> Result<RequestAdapter<ListAction>, DaemonError> {
        Ok(
            RequestAdapter(
                ListAction {
                    id: context.get("id")?.to_string()?,
                    path: context.get("path")?.to_string()?,
                }
            )
        )
    }
}

impl Request for RequestAdapter<ListAction> {
    fn process(&self, container: &mut Container) -> Result<Vec<u8>, DaemonError> {
        let response = match container.read_dir(
            normalize(
                Path::new(&self.0.path)
            ).as_path()
        ){
            Ok(collection) =>
                (Response {
                    id: self.0.id.clone(),
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
                    id: self.0.id.clone(),
                    kind: ResponseKind::Collection,
                    status: ResponseStatus::Fail,
                    content: None,
                    error: Some(format!("{}", DaemonError::from(error)))
                }).encode()?
        };
        Ok(response)
    }
}
