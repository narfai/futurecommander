mod utils;
mod errors;
mod message_delta;
mod codec;
mod context;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::{ prelude::* };

pub use self::{
    message_delta::MessageDelta,
    codec::Codec,
    context::RustMessageContext
};
