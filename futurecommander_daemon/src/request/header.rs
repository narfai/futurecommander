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
use crate:: {
    errors::DaemonError,
    Context
};

use std::{
    fmt::{
        Display,
        Formatter,
        Result as FmtResult
    },
};

use serde::{
    Serialize,
    Deserialize
};

use bincode::{ deserialize, serialize };

use crate::{
    request::{
        Request,
        RequestAdapter,
        ListAction
    }
};

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum RequestHeader {
    LIST
}

impl Eq for RequestHeader {}

impl RequestHeader {
    /** Lifecycle step 1 - Consumer - create from action name string **/
    pub fn new(s: &str) -> Result<RequestHeader, DaemonError> {
        match s {
            t if t == RequestHeader::LIST.to_string() => Ok(RequestHeader::LIST),
            _ => Err(DaemonError::InvalidRequest)
        }
    }

    /** Lifecycle step 2 - Consumer - binary encode body from Context object**/
    pub fn encode_adapter(self, context: Context) -> Result<Vec<u8>, DaemonError> {
        let mut binary_request = vec![self as u8];
        match self {
            RequestHeader::LIST => {
                binary_request.append(
                    &mut serialize(&ListAction::adapter(context)?)?
                )
            }
        }
        Ok(binary_request)
    }

    /** Lifecycle step 3 - Daemon - read size of the header as soon as request is emitted **/
    pub fn len() -> usize {
        1 as usize
    }

    /** Lifecycle step 4 - Daemon - retrieve proper header from those bytes **/
    pub fn parse(bytes: &[u8]) -> Result<Self, DaemonError> {
        if let Some(byte) = bytes.first() {
            match byte {
                b if b == &(RequestHeader::LIST as u8) => Ok(RequestHeader::LIST),
//                b if b == &(RequestHeader::Status as u8) => Ok(RequestHeader::Status),
                _ => Err(DaemonError::InvalidRequest)
            }
        } else {
            Err(DaemonError::InvalidRequest)
        }
    }

    /** Lifecycle step 5 - Daemon - decode bytes left from response weather header kind **/
    pub fn decode_adapter(self, bytes: &[u8]) -> Result<Box<Request>, DaemonError> {
        match self {
            RequestHeader::LIST => {
                let request: RequestAdapter<ListAction> = deserialize(bytes)?;
                Ok(Box::new(request))
            }
        }
    }
}

impl Display for RequestHeader {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

#[cfg_attr(tarpaulin, skip)]
#[cfg(test)]
mod tests {
    use super::*;

    use futurecommander_filesystem::{
        Container,
        sample::Samples
    };

    use crate::{
        ContextString
    };

    #[test]
    fn test_header_codec_list(){
        /* Consumer */
        let id = "jsvs2qz26".to_string();
        let path = Samples::static_samples_path().to_string_lossy().to_string();

        let mut context = Context::default();
        context.set("id", Box::new(ContextString::from(id.clone())));
        context.set("path", Box::new(ContextString::from(path.clone())));

        let header = RequestHeader::new("LIST").unwrap();

        assert_eq!(header, RequestHeader::LIST);

        let binary_request = header.encode_adapter(context).unwrap();

        /* Daemon */
        let payload = binary_request.as_slice();
        let decoded_header = RequestHeader::parse(&payload[..RequestHeader::len()]).unwrap();

        assert_eq!(decoded_header, RequestHeader::LIST);
    }
}
