mod router;
mod asset;
mod prelude;

pub use macros;
use id::{Id, Keys};

// note this numbers are generated from some third party website
// so these keys aren't 100% secure, but it should be sufficient for this app
const KEYS: Keys = (
    0xcb543806d99c8d5d, 
    0x2dc46f11ab6627e0, 
    0x422656cf8a78f7f2, 
    0x49177b25554b5887
);


// library's init function call inside of project/lib.rs
pub fn init() {
    Id::init(KEYS);
    let id = Id::new();
    log::debug!("{}", id.to_hex());
}