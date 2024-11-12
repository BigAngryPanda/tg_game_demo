use std::fmt::Debug;

use wasm_bindgen::JsValue;

pub fn write<T: ToString>(data: &T) {
    web_sys::console::log(&web_sys::js_sys::Array::from(&JsValue::from_str(&data.to_string())));
}

pub fn write_debug<T: Debug>(data: &T) {
    write(&format!("{:?}", data));
}