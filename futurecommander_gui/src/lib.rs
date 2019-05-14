//mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::prelude::*;
use futurecommander_daemon::{
    Request,
    Response,
    ListRequest
};

#[wasm_bindgen]
extern "C" {
    pub type JsEntry;

    #[wasm_bindgen(constructor)]
    pub fn new_entry(name: Option<&str>, is_dir: bool, is_file: bool) -> JsEntry;

    pub type JsResponse;

    #[wasm_bindgen(constructor)]
    pub fn new_response(id: &str, status: &str, kind: &str, error: Option<&str>) -> JsResponse;

    #[wasm_bindgen(method)]
    pub fn add_entry(this: &JsResponse, entry: JsEntry);
}

pub trait ResponsePacket {
    fn into_response(self) -> JsResponse;
}

impl ResponsePacket for Response {
    fn into_response(self) -> JsResponse {
        let response = JsResponse::new_response(
            self.id.as_str(),
            self.status.as_str(),
            self.kind.as_str(),
            if let Some(error_str) = &self.error {
                Some(error_str.as_str())
            } else { None }
        );

        if let Some(content) = self.content {
            for entry in content {
                response.add_entry(
                    JsEntry::new_entry(
                        if let Some(name) = &entry.name {
                            Some(name.as_str())
                        } else {
                            None
                        },
                        entry.is_dir,
                        entry.is_file
                    )
                );
            }
        }

        response
    }
}

#[wasm_bindgen]
pub fn list(id : &str, path: &str) -> Box<[u8]> {
    let mut request : Vec<u8> = vec![ListRequest::header() as u8];
    request.append(
        &mut ListRequest::new(id, path)
            .as_bytes()
            .unwrap()
    );

    request.into_boxed_slice()
}

#[wasm_bindgen]
pub fn decode(response: &[u8]) -> Result<JsResponse, JsValue> {
    match Response::decode(response) {
        Ok(decoded) => Ok(decoded.into_response()),
        Err(error) => Err(JsValue::from_str(format!("{}", error).as_str()))
    }
}
