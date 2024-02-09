mod router;
mod asset;
mod id;
pub mod prelude;

pub use macros;
pub use asset::event;

use crate::id::{Id, IdLike};


// library's init function call inside of project/lib.rs
pub fn init() {
    let id = Id::default();
    log::debug!("{}", id.to_hex());
}