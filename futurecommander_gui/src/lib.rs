//mod utils;

use wasm_bindgen::prelude::*;
use futurecommander_daemon::{
    Request,
    Response,
    ResponseKind,
    ResponseStatus,
    SerializableEntry
};

use std::{
    str::{
        from_utf8
    }
};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub trait JsResponsible {
    fn into_response(self) -> JsResponse;
}

impl JsResponsible for Response {
    fn into_response(self) -> JsResponse {
        let mut entries = Vec::new();
        let error = {
            (match &self.error { //TODO JS enums ?
                Some(error) => error.as_bytes(),
                None => "".as_bytes()
            }).iter().cloned().collect()
        };

        let kind = { //TODO JS enums
            (match &self.kind {
                ResponseKind::Collection => "collection".as_bytes(),
                ResponseKind::Entry => "entry".as_bytes(),
                ResponseKind::String => "string".as_bytes()
            }).iter().cloned().collect()
        };

        let status = {
            (match &self.status { //TODO JS enums
                ResponseStatus::Success => "success".as_bytes(),
                ResponseStatus::Fail => "fail".as_bytes(),
                ResponseStatus::Exit => "exit".as_bytes()
            }).iter().cloned().collect()
        };

        for entry in self.content {
            entries.push(JsEntry {
                path: entry.path.to_string_lossy().as_bytes().iter().cloned().collect(),
                name: match &entry.name {
                    Some(name) => Some(name.as_bytes().iter().cloned().collect()),
                    None => None
                },
                is_dir: entry.is_dir,
                is_file: entry.is_file,
                exists: entry.exists
            })
        }

        JsResponse {
            id: self.id,
            kind,
            status,
            entries,
            error,
            cursor: 0
        }
    }
}

type RequestId = Vec<u8>;

#[wasm_bindgen]
#[derive(Clone)]
pub struct JsResponse{
    id: RequestId,
    status: Vec<u8>,
    kind: Vec<u8>,
    error: Vec<u8>,
    entries: Vec<JsEntry>,
    cursor: usize
}

#[wasm_bindgen]
impl JsResponse {
    pub fn id(&self) -> Result<String, JsValue> {
        match from_utf8(self.id.as_slice()) {
            Ok(id_str) => Ok(id_str.to_string()),
            Err(error) => Err(JsValue::from_str(format!("{}", error).as_str()))
        }
    }

    pub fn status(&self) -> Result<String, JsValue> {
        match from_utf8(self.status.as_slice()) {
            Ok(status_str) => Ok(status_str.to_string()),
            Err(error) => Err(JsValue::from_str(format!("{}", error).as_str()))
        }
    }

    pub fn kind(&self) -> Result<String, JsValue> {
        match from_utf8(self.kind.as_slice()) {
            Ok(kind_str) => Ok(kind_str.to_string()),
            Err(error) => Err(JsValue::from_str(format!("{}", error).as_str()))
        }
    }

    pub fn error(&self) -> Result<String, JsValue> {
        match from_utf8(self.error.as_slice()) {
            Ok(error_str) => Ok(error_str.to_string()),
            Err(error) => Err(JsValue::from_str(format!("{}", error).as_str()))
        }
    }

    pub fn next(&mut self) -> Option<JsEntry> {
        match self.entries.get(self.cursor) {
            Some(entry) => {
                self.cursor += 1;
                Some(entry.clone())
            },
            None => None
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct JsEntry{
    path: Vec<u8>,
    name: Option<Vec<u8>>,
    is_dir: bool,
    is_file: bool,
    exists: bool
}

#[wasm_bindgen]
impl JsEntry {
    pub fn path(&self) -> Result<String, JsValue> {
        match from_utf8(self.path.as_slice()) {
            Ok(path_str) => Ok(path_str.to_string()),
            Err(error) => Err(JsValue::from_str(format!("{}", error).as_str()))
        }
    }

    pub fn name(&self) -> Result<Option<String>, JsValue> {
        match &self.name {
            Some(bytes) =>
                match from_utf8(bytes.as_slice()) {
                    Ok(name_str) => Ok(Some(name_str.to_string())),
                    Err(error) => Err(JsValue::from_str(format!("{}", error).as_str()))
                },
            None => Ok(None)
        }
    }

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    pub fn is_file(&self) -> bool {
        self.is_file
    }

    pub fn exists(&self) -> bool {
        self.exists
    }
}

#[wasm_bindgen]
pub fn list(id : &str, path: &str) -> Box<[u8]> {
    Request::List {
        id: id.as_bytes().iter().cloned().collect(),
        path: path.to_string()
    }.into_bytes()
    .unwrap()
    .into_boxed_slice()
}

#[wasm_bindgen]
pub fn decode(response: &[u8]) -> Result<JsResponse, JsValue> {
    match Response::decode(response) {
        Ok(decoded) => Ok(decoded.into_response()),
        Err(error) => Err(JsValue::from_str(format!("{}", error).as_str()))
    }
}
