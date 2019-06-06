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
    path::{ Path, PathBuf }
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
pub struct CreateFileAction {
    pub path: PathBuf,
    pub recursive: bool,
    pub overwrite: bool
}


impl CreateFileAction {
    pub fn adapter(context: Context) -> Result<RequestAdapter<CreateFileAction>, DaemonError> {
        Ok(
            RequestAdapter::new(
                    context.get("id")?.to_string()?.as_str(),
                    CreateFileAction {
                        path: normalize(
                            Path::new(context.get("path")?.to_string()?.as_str())
                        ),
                        recursive: context.get("recursive")?.to_bool()?,
                        overwrite: context.get("overwrite")?.to_bool()?,
                }
            )
        )
    }
}

impl Request for RequestAdapter<CreateFileAction> {
    fn process(&self, container: &mut Container) -> Result<Box<dyn Response>, DaemonError> {
        let collection = container.read_dir(
            normalize(
                Path::new(&self.inner.path)
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
