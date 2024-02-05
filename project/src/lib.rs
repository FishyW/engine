/*
THIS CODE IS AUTO GENERATED, DO NOT MODIFY ANYTHING IN THIS FILE
(except if ur a dev :\)
*/

mod engine {
    pub mod prelude;
}

use crate::engine::prelude::*;

declare!();

#[wasm_bindgen(js_namespace = console)]
extern "C" {
    fn log(s: &str);
}

#[wasm_bindgen(js_namespace = hello)]
extern "C" {
    fn add_world(s: &str) -> String;
}

#[wasm_bindgen]
pub fn init_script() -> String {
    return add_world("Hello");
}
