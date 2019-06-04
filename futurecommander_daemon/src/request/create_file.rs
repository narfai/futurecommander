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
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playground(){

    }
}
//
//#[cfg_attr(tarpaulin, skip)]
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    use futurecommander_filesystem::{
//        sample::Samples
//    };
//
//    use crate::{
//        ContextString
//    };
//
//    fn assert_static_sample_success_response(collection: &[SerializableEntry]){
//        let a = &collection[0];
//        assert_eq!(a.name, Some("A".to_string()));
//        assert_eq!(a.is_dir, true);
//        assert_eq!(a.is_file, false);
//
//        let b = &collection[1];
//        assert_eq!(b.name, Some("B".to_string()));
//        assert_eq!(b.is_dir, true);
//        assert_eq!(b.is_file, false);
//
//        let f = &collection[2];
//        assert_eq!(f.name, Some("F".to_string()));
//        assert_eq!(f.is_dir, false);
//        assert_eq!(f.is_file, true);
//    }
//
//    #[test]
//    fn transform_context_to_list_request(){
//        let id = "jsvs2qz20".to_string();
//        let path = Samples::static_samples_path().to_string_lossy().to_string();
//        let mut context = Context::default();
//        let mut container = Container::default();
//
//        context.set("id", Box::new(ContextString::from(id.clone())));
//        context.set("path", Box::new(ContextString::from(path.clone())));
//
//        let action = CreateFileAction::adapter(context).unwrap();
//
//        let response = Response::decode(
//            action.process(&mut container)
//                .unwrap().as_slice()
//        ).unwrap();
//
//        assert_eq!(response.id, id);
//        assert_eq!(response.kind, ResponseKind::Collection);
//        assert_eq!(response.status, ResponseStatus::Success);
//
//        assert_static_sample_success_response(response.content.unwrap().as_slice());
//    }
//
//    #[test]
//    fn process_list_request_success(){
//        let id = "jsvs2qz20".to_string();
//        let path = Samples::static_samples_path().to_string_lossy().to_string();
//        let mut container = Container::default();
//        let action = RequestAdapter(
//            CreateFileAction {
//                id: id.clone(),
//                path,
//            }
//        );
//
//        let response = Response::decode(
//            action.process(&mut container)
//                .unwrap().as_slice()
//        ).unwrap();
//
//        assert_eq!(response.id, id);
//        assert_eq!(response.kind, ResponseKind::Collection);
//        assert_eq!(response.status, ResponseStatus::Success);
//
//        assert_static_sample_success_response(response.content.unwrap().as_slice());
//    }
//
//    #[test]
//    fn process_list_request_failed(){
//        let id = "jsvs2qz21".to_string();
//        let path = Samples::static_samples_path()
//            .join("WILL_NEVER_EXISTS")
//            .to_string_lossy().to_string();
//
//        let mut container = Container::default();
//        let action = RequestAdapter(
//            CreateFileAction {
//                id: id.clone(),
//                path,
//            }
//        );
//
//        let response = Response::decode(
//            action.process(&mut container)
//                .unwrap().as_slice()
//        ).unwrap();
//
//        assert_eq!(response.id, id);
//        assert_eq!(response.kind, ResponseKind::Collection);
//        assert_eq!(response.status, ResponseStatus::Fail);
//
//        assert!(response.error.unwrap().contains("does not exists"));
//    }
//}
