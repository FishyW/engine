use std::rc::Weak;

use crate::asset::*;

macro_rules! addressify {
    // for Objects
    ($var:expr) => {
        $var::type_address()
    };
    // for instances
    (!$var:expr) => {
        $var.address()
    };
}

// address macro
// create a list of addresses from assets
// you can only pass an address to assets
macro_rules! address {
    ($($var:expr),*) => {
        vec![$(addressify!($var)),*].into()
    }
}

pub trait Address<T: Event> {
    fn receivers(&self) -> Vec<Weak<dyn Receiver<T>>>;
}


pub(in crate::asset) use {address};