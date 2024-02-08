// ahash is fast for smaller keys, which is ideal

use std::sync::atomic::{AtomicUsize, Ordering};

use ahash::RandomState;

static mut HASHER: Option<RandomState> = None;

#[derive(Debug, Hash, Clone, Copy)]
pub struct Id {
    id: u64
}

pub type Keys = (u64, u64, u64, u64);


impl Id {
    // call init at the very beginning
    pub fn init(key: Keys) {
        unsafe {
            HASHER = Some(RandomState::with_seeds(key.0, key.1, key.2, key.3));
        }
    }

    pub fn new() -> Self {
        // hash is used to ensure that the id is unique across
        // Rust Tauri and Rust macro
        let hasher = unsafe{ HASHER.as_ref()
            .expect("Id::init() not called!") };

        static COUNTER:AtomicUsize = AtomicUsize::new(1);
        let value = COUNTER.fetch_add(1, Ordering::Relaxed);
        Id {id: hasher.hash_one(value)}
    }

    // converts the hash to hexadecimal string
    pub fn to_hex(&self) -> String {
        format!("{:x}", self.id)
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Id {}