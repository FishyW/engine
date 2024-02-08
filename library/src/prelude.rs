use std::{cell::RefCell, rc::Weak};

pub use crate::asset::{Component, Object, Receiver, Event, Address};
pub use ahash::{HashMap, HashMapExt};
pub use id::Id;

pub type StaticWeakMap<T> = RefCell<HashMap<Id, Weak<dyn Receiver<T>>>>;