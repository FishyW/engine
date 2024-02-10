use core::event::ClickEvent;

use prelude::Event;
use wasm_bindgen::prelude::*;

mod router;
mod asset;
mod id;
pub mod prelude;

pub use macros;

pub mod core;


#[wasm_bindgen]
pub fn event_receive(name: &str) {
    if name == "click" {
        ClickEvent.broadcast()
    }
}

// library's init function call inside of project/lib.rs
pub fn init() {
    wasm_logger::init(wasm_logger::Config::default());
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}