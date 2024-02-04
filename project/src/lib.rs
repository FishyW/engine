/*
THIS CODE IS AUTO GENERATED, DO NOT MODIFY ANYTHING IN THIS FILE
(except if ur a dev :\)
*/

use crate::engine::prelude::*;

// auto generate code that makes it possible to 
// do use components::Hello;
engine_lib::macros
    ::generate_project_imports!();


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

