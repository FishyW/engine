// library implementation of Id


use std::fmt;

use id::{IdBuilder,  Keys};
pub use id::IdLike;

// note this numbers are generated from some third party website
// so these keys aren't 100% secure, but it should be sufficient for this app
const KEYS: Keys = (
    0xcb543806d99c8d5d, 
    0x2dc46f11ab6627e0, 
    0x422656cf8a78f7f2, 
    0x49177b25554b5887
);

const ID_BUILDER: IdBuilder = IdBuilder::new(KEYS);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Id {
    value: u64
}



impl IdLike for Id {
    fn new(value: u64) -> Self {
        Id{value}
    }

    fn value(&self) -> u64 {
        self.value
    }
    
}

impl Default for Id {
    fn default() -> Self {
        ID_BUILDER.id()
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}