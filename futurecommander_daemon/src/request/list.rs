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
    request::{
        Request,
        RequestAdapter

    },
    response::{
        Response,
        ResponseStatus,
        ResponseAdapter,
        EntriesResponse,
        ResponseHeader
    },
    context::{
        Context,
    }

};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListAction {
    pub path: String //TODO store as path
}

impl ListAction {
    pub fn adapter(context: Context) -> Result<RequestAdapter<ListAction>, DaemonError> {
        Ok(
            RequestAdapter::new(
                context.get("id")?.to_string()?.as_str(),
                ListAction {
                    path: context.get("path")?.to_string()?,//TODO convert to path
                }
            )
        )
    }
}

impl Request for RequestAdapter<ListAction> {
    fn process(&self, container: &mut Container) -> Result<Box<dyn Response>, DaemonError> {
        let collection = container.read_dir(
            normalize(
                Path::new(self.inner.path.as_str())
            ).as_path()
        )?;

        Ok(
            Box::new(
                ResponseAdapter::new(
                self.id.as_str(),
                ResponseStatus::Success,
                    ResponseHeader::Entries,
                    EntriesResponse(
                        Some(
                            collection
                                .into_iter()
                                .map(|entry| SerializableEntry::from(&entry))
                                .collect::<Vec<SerializableEntry>>()
                        )
                    )
                )
            )
        )
    }

    fn id(&self) -> &str {
        self.id.as_str()
    }
}
