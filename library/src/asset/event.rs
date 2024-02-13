use std::{cell::Ref, ops::Add, rc::Weak};

use ahash::HashMap;

use crate::asset::*;


/// Create a list of addresses from assets
/// usage: 
/// ```no_run
/// address!(MyObject, MyObject2, !object);
/// ```
/// MyObject and MyObject2 are object types.
/// To pass in an object instance use the `!` operator.
/// The object instance should be a reference (ie. `&MyObject`),
/// do not pass in an `Rc<RefCell<instance>>`
#[macro_export]
macro_rules! address {
    // using tt muncher pattern
    (@main @$var:ident !$addr:expr, $($rest:tt)*) => {
        let $var = AddressBuilder::add($var, $addr);
        address!(@main @$var $($rest)*);
    };

    (@main @$var:ident !$addr:expr) => {
        let $var = AddressBuilder::add($var, $addr);
    };

    (@main @$var:ident $($addr:ident)::*, $($rest:tt)*) => {
        let $var = AddressBuilder::add($var, $($addr)::*::Address());
        address!(@main @$var $($rest)*);
    };

    (@main @$var:ident $($addr:ident)::*) => {
        let $var = AddressBuilder::add($var, $($addr)::*::Address());
    };

    ($($rest:tt)*) => {
        {
            let builder = AddressBuilder::new();
            address!(@main @builder $($rest)*);
            builder
        }
    }
}


// 'static means event must be implemented for an owned type
pub trait Event: Clone + 'static {

    fn send(self, target: impl Address<Self>);
    
    fn broadcast(self);

    // register an address to this event
    // this iterates through all registers specified by the address
    // and registers each register to the event
    fn register(item: impl Register<Self> + 'static);

    // cleanup function when scene changes
    fn clear(self);
}

pub trait EventUnsized {}

impl <T: Event> EventUnsized for T {}


pub trait Receiver<T: Event> {
    fn receive(&mut self, event: T);

}

// an address gives a vector of registers, which are usually assets
// that has been programmed to receive a certain event
pub trait Address<T: Event> {
    // needs RefCell since receive() borrows &mut self -> needs a mutable borrow
    // or receivers
    fn receivers<'a>(&'a self) -> Vec<Rc<RefCell<dyn Receiver<T> + 'a>>>;
}

// Registers are any assets or asset types that can be registered
// note that Register and Receiver are different types
// since an object type can be registered, but an object type doesn't have a receiver
// Address and Register is different, since an Object Group can't be registered
// but an Object Group can made into an address
pub trait Register<T: Event>: Address<T> {
     // get the id used to register
     fn register_id(&self) -> Id;
}



// Implement Address for an asset instance (Rc<RefCell<U>>)
impl <T: Event, U: Receiver<T> + Asset> Address<T> for Rc<RefCell<U>> {
    // takes ownership of the address
    fn receivers<'a>(&'a self) -> Vec<Rc<RefCell<dyn Receiver<T> + 'a>>> {
        vec![Rc::clone(&self) as Rc<RefCell<dyn Receiver<T>>>]
    }
}



// creates an address by creating a vector of addresses
pub struct AddressBuilder<'a, T: Event> {
    addresses: Vec<Box<dyn Address<T> + 'a>>
}

impl <'a, T: Event> AddressBuilder<'a, T> {
    pub fn new() -> Self {
        AddressBuilder{addresses: vec![]}
    }

    pub fn add(mut self, address: impl Address<T> + 'a) -> Self {
        self.addresses.push(Box::new(address));
        self
    }
}

impl <'a, T: Event> Address<T> for AddressBuilder<'a, T>{
    fn receivers<'b>(&'b self) -> Vec<Rc<RefCell<dyn Receiver<T> + 'b>>> {
        self.addresses.iter().fold(vec![], |mut acc, addr| {
            acc.append(&mut addr.receivers());
            acc
        })
    }
}
