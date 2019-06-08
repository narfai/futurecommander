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

mod header;
mod error;
mod entries;

pub use self::{
    header::ResponseHeader,
    error::ErrorResponse,
    entries::EntriesResponse
};

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



use crate::{
    errors::DaemonError
};


#[derive(Serialize, PartialEq, Deserialize, Debug, Copy, Clone)]
pub enum ResponseStatus {
    Success,
    Fail
}

impl Eq for ResponseStatus {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseAdapter<T: Serialize>{
    header: ResponseHeader,
    body: T,
    id: String,
    status: ResponseStatus
}

impl <T: Serialize>ResponseAdapter<T> {
    pub fn new(id: &str, status: ResponseStatus, header: ResponseHeader, body: T) -> ResponseAdapter<T> {
        ResponseAdapter {
            id: id.to_string(),
            status,
            header,
            body
        }
    }

    pub fn inner(self) -> T {
        self.body
    }
}

pub trait Response : Debug {
    fn encode(&self) -> Result<Vec<u8>, DaemonError>;
}

#[typetag::serde(tag = "type")]
pub trait SerializableResponse : Debug {
    fn serializable(&self) -> Box<SerializableResponse>;
}
//
//#[cfg_attr(tarpaulin, skip)]
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    use std::{
//        path::{ Path }
//    };
//
//    use futurecommander_filesystem::{
//        EntryAdapter,
//        SerializableEntry
//    };
//
//    #[test]
//    fn test_codec_response(){
//        let response = Response {
//            id: "jsvsazd21".to_string(),
//            kind: ResponseKind::Collection,
//            status: ResponseStatus::Success,
//            content: Some(vec![SerializableEntry::from(&EntryAdapter(Path::new("/test/directory")))]),
//            error: None
//        };
//
//        let codec_response = Response::decode(
//            response.encode()
//                .unwrap()
//                .as_slice()
//        ).unwrap();
//
//        assert_eq!(codec_response.id, "jsvsazd21".to_string());
//        assert_eq!(codec_response.kind, ResponseKind::Collection);
//        assert_eq!(codec_response.status, ResponseStatus::Success);
//        assert_eq!(codec_response.content, Some(vec![SerializableEntry::from(&EntryAdapter(Path::new("/test/directory")))]));
//        assert_eq!(codec_response.error, None);
//    }
//}
