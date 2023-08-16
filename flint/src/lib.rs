use js_sys::JsString;
use wasm_bindgen::prelude::*;
use caveman::info;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn libcaveman_info() -> JsString {
    JsString::from(info())
}
