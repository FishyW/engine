/*
THIS CODE IS AUTO GENERATED, DO NOT MODIFY ANYTHING IN THIS FILE
(except if ur a dev :\)
*/

pub mod engine {
    pub mod prelude;
}

use crate::engine::prelude::*;

declare!("src/components");
declare!("src/actions");
declare!("src/events");
declare!("src/managers");
declare!("src/objects");

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


