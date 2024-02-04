use crate::engine::prelude::*;


#[wasm_bindgen]
pub fn init_test() {
    // te
}
struct MyObject {
	custom_prop1: &'static str,
	custom_prop2: &'static str,
    x: i32
}

fn main() {
    MyObject{"hello", "world"};
}

