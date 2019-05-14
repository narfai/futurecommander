//mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::{
    prelude::*
};

use futurecommander_daemon::{
    Request,
    Response,
    ListRequest,
    RequestHeader
};

mod errors;

//TODO Light iterate over values instead of double copy : https://rustwasm.github.io/docs/wasm-bindgen/reference/iterating-over-js-values.html
//TODO could be usefull for display apply loaders : https://rustwasm.github.io/docs/wasm-bindgen/reference/receiving-js-closures-in-rust.html
//TODO the whole client could be in rust and return promises : https://rustwasm.github.io/docs/wasm-bindgen/reference/js-promises-and-rust-futures.html and https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/start.html and https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/skip.html for obscure filesystem internals


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Request)]
    pub type JsRequest;
    #[wasm_bindgen(method)]
    pub fn get_id(this: &JsRequest) -> String;

    #[wasm_bindgen(method)]
    pub fn get_type(this: &JsRequest) -> String;

    #[wasm_bindgen(method)]
    pub fn get_parameter(this: &JsRequest, key: &str) -> Option<String>;
}

impl From<&JsRequest> for Result<ListRequest, errors::AddonError> {
    fn from(js_request: &JsRequest) -> Self {
        match js_request.get_parameter("path") {
            Some(path) => Ok(
                ListRequest::new(
                js_request.get_id().as_str(),
                path.as_str())
            ),
            None => Err(errors::AddonError::InvalidArgument("path".to_string()))
        }
    }
}

#[wasm_bindgen]
pub fn request(request: &JsRequest) -> Result<Box<[u8]>, JsValue> {
    fn encode_request(request: &JsRequest) -> Result<Box<[u8]>, errors::AddonError> {
        match request.get_type().as_str() {
            key if RequestHeader::list() == key => Ok(
                Result::<ListRequest, errors::AddonError>::from(request)?
                .as_bytes()?
                .into_boxed_slice()
            ),
            _ => Err(
                errors::AddonError::InvalidRequest(request.get_type())
            ),
        }
    }

    match encode_request(request) {
        Ok(result) => Ok(result),
        Err(error) => Err(error.into())
    }
}

#[wasm_bindgen]
pub fn decode(response: &[u8]) -> Result<JsValue, JsValue> {
    fn decode_response(response: &[u8]) -> Result<JsValue, errors::AddonError> {
        Ok(JsValue::from_serde(&Response::decode(response)?)?)
    }

    match decode_response(response) {
        Ok(result) => Ok(result),
        Err(error) => Err(error.into())
    }
}
