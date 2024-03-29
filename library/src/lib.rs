use router::Router;
use structs::event::{Event, RandomEvent};

use wasm_bindgen::prelude::*;

pub use macros;

mod router;
mod structs;

pub fn init() {
    // assigns the event to the router
    RandomEvent::new();
}

#[wasm_bindgen]
pub fn names() -> Vec<String> {
    let val = Router::instance().names();

    val
}

#[wasm_bindgen]
pub fn add_event() {
    RandomEvent::new();
}
