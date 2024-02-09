// ahash is fast for smaller keys, which is ideal

use std::sync::atomic::{AtomicUsize, Ordering};

use ahash::RandomState;

pub type Keys = (u64, u64, u64, u64);

use std::hash::Hash;

pub struct IdBuilder {
    hasher: RandomState
}

impl IdBuilder {
    // call init at the very beginning
    pub const fn new(key: Keys) -> Self {
        let hasher = RandomState::with_seeds(key.0, key.1, key.2, key.3);
        IdBuilder{hasher}
    }

    // id is now a trait
    // this is so that a custom Default can be implemented 
    pub fn id<T: IdLike>(&self) -> T {
        static COUNTER:AtomicUsize = AtomicUsize::new(1);
        let value = COUNTER.fetch_add(1, Ordering::Relaxed);
        T::new(self.hasher.hash_one(value))
    }
}

pub trait IdLike: Hash + Clone + Copy + Eq + PartialEq {
    fn new(value: u64) -> Self;
    fn value(&self) -> u64;

    // converts the hash to hexadecimal string
    fn to_hex(&self) -> String {
        format!("{:x}", self.value())
    }
}

