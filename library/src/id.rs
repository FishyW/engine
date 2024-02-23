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
    value: u64,
    is_empty: bool
}

impl Id {
    pub fn empty() -> Id {
        Id {value: 0, is_empty: true}
    }

    pub fn is_empty(&self) -> bool {
        self.is_empty
    }
}


impl IdLike for Id {
    fn new(value: u64) -> Self {
        Id{value, is_empty: false}
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct TypeId {
    value: Id
}

impl TypeId {
    pub fn empty() -> TypeId {
        TypeId {value: Id::empty()}
    }

    pub fn is_empty(&self) -> bool {
        Id::is_empty(&self.value)
    }
}



impl fmt::Debug for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value.to_hex())
    }
}

