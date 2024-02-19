use core::{component::Transform, event::ClickEvent};

use prelude::{Component, Event};
use wasm_bindgen::prelude::*;

// where the Event Router lies
pub mod router;

// code for various assets -> Object, Component, Event, etc.
mod asset;

// Id initialization
mod id;

// Module that defines project prelude, ie. stuff that is imported to the project
// by default
pub mod prelude;

pub use macros;

// Standard Library Assets (such as Transform)
pub mod core;


#[wasm_bindgen]
pub fn event_receive(name: &str) {
    if name == "click" {
        ClickEvent.broadcast();
    }
}

// library's init function call inside of project/lib.rs
pub fn init() {
    #[cfg(debug_assertions)]
    {
        wasm_logger::init(wasm_logger::Config::default());
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }
}