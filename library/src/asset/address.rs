use std::{ops::Add, rc::Weak};

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


// Registers are any assets or asset types that can be registered
// note that Register and Receiver are different types
// since an object type can be registered, but an object type doesn't have a receiver
// Address and Register is different, since an Object Group can't be registered
// but an Object Group can made into an address
pub trait Register<T: Event>: 'static {
    // Rc instead of Weak, implementers of Registerable 
    // must guarantee that the receiver in question exists
    fn receiver(&self) -> Vec<Rc<dyn Receiver<T>>>;

    // get the id used to register
    fn register_id(&self) -> Id;

    fn to_ref(&self) -> Rc<Self>
        where Self:Sized;

    // get a weak reference of itself
    fn weak_ref(&self) -> Weak<Self> 
    where Self:Sized {
        // the strong reference goes out of scope at the end
        // and is implicitly dropped
        let strong_ref = self.to_ref();
        Rc::downgrade(&strong_ref)
    }
        
}

// an address gives a vector of registers, which are usually assets
// that has been programmed to receive a certain event
pub trait Address<T: Event>: 'static {
    fn registers(&self) -> Vec<Rc<dyn Register<T>>>;
}

// Registers can also be used as Addresses
// this blanket implementation makes this possible
impl <T: Event, U: Register<T>> Address<T> for U {
    fn registers(&self) -> Vec<Rc<dyn Register<T>>> {
        vec![self.to_ref()]
    }
    
}

pub(in crate::asset) use { address };
