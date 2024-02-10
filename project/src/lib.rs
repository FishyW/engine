/*
ONLY MODIFY THIS FILE IF YOU WANT TO ADD YOUR OWN SUBMODULES
DO NOT MODIFY ANYTHING ELSE BESIDES THAT
*/

pub mod engine {
    pub mod prelude;
    use lib::macros::declare;
    declare!("src/engine/scene");
}

use crate::engine::prelude::*;


// module and submodule declarations
declare!("src/components", {
    // declare a submodule called hello
    declare!("src/components/hello");
});

declare!("src/actions");
declare!("src/events");
declare!("src/managers");
declare!("src/objects");

#[wasm_bindgen(js_namespace = console)]
extern "C" {
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn init_start()  {
    lib::init();
}

#[asset(object)]
#[include()]
struct Test;


