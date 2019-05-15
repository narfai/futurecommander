//mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::{
    prelude::*
};

use futurecommander_daemon::{
    DaemonError,
    Request,
    Response,
    RequestHeader,
    Context,
    ContextType
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
    pub fn get_type(this: &JsRequest) -> String;

    #[wasm_bindgen(method)]
    pub fn next_key(this: &JsRequest) -> Option<String>;

    #[wasm_bindgen(method)]
    pub fn get_parameter(this: &JsRequest, key: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Clone)]
struct ContextValueWrapper(pub JsValue, pub String);

impl ContextType for ContextValueWrapper {
    fn to_bool(&self) -> Result<bool, DaemonError> {
        if self.0.is_null() {
            return Err(DaemonError::ContextValueDoesNotExists(self.1.clone()))
        }

        if let Some(b) = self.0.as_bool() {
            Ok(b)
        } else {
            Err(DaemonError::ContextCannotCast("JsValue".to_string(), "bool".to_string()))
        }
    }

    fn to_string(&self) -> Result<String, DaemonError> {
        if self.0.is_null() {
            return Err(DaemonError::ContextValueDoesNotExists(self.1.clone()))
        }

        if let Some(s) = self.0.as_string() {
            Ok(s)
        } else {
            Err(DaemonError::ContextCannotCast("JsValue".to_string(), "string".to_string()))
        }
    }

    fn box_clone(&self) -> Box<dyn ContextType> {
        Box::new(self.clone())
    }
}

impl From<&JsRequest> for Context {
    fn from(js_request: &JsRequest) -> Self {
        let mut context = Context::default();
        while let Some(key) = js_request.next_key() {
            context.set(
                key.as_str(),
                Box::new(
                    ContextValueWrapper(
                        js_request.get_parameter(key.as_str()),
                        key.clone()
                    )
                )
            )
        }
        context
    }
}

#[wasm_bindgen]
pub fn request(request: &JsRequest) -> Result<Box<[u8]>, JsValue> {
    fn encode_request(request: &JsRequest) -> Result<Box<[u8]>, errors::AddonError> {
        let context = Context::from(request);
//        log(format!("{:?}", context.debug_keys()).as_str());
        Ok(
            RequestHeader::new(request.get_type().as_str())?
                .encode_adapter(context)?
                .into_boxed_slice()
        )
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
