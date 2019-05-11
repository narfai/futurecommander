#[no_mangle]
pub extern fn __cxa_pure_virtual() {
    loop{};
}


#[macro_use]
extern crate neon;

use neon::prelude::*;

fn hello_world(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello world !"))
}

register_module!(mut cx, {
    cx.export_function("hello_world", hello_world)
});
