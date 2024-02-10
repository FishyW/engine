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

    fn send(self, target: impl Address<Self>) 
        where Self:Sized;

    fn broadcast(self) 
        where Self:Sized;

    // register an address to this event
    // this iterates through all registers specified by the address
    // and registers each register to the event
    fn register(item: impl Address<Self>);

    // cleanup function
    fn clear(self);

}

pub trait Receiver<T: Event>: 'static{
    fn receive(&mut self, event: T);
}

// an address gives a vector of registers, which are usually assets
// that has been programmed to receive a certain event
pub trait Address<T: Event> {
    fn registers(self) -> Vec<Rc<dyn Register<T>>>;
}

// Registers are any assets or asset types that can be registered
// note that Register and Receiver are different types
// since an object type can be registered, but an object type doesn't have a receiver
// Address and Register is different, since an Object Group can't be registered
// but an Object Group can made into an address
pub trait Register<T: Event>: 'static {
    // Rc instead of Weak, implementers of Registerable 
    // must guarantee that the receiver in question exists
    fn receivers(&self) -> Vec<Rc<RefCell<dyn Receiver<T>>>>;

     // get the id used to register
     fn register_id(&self) -> Id;
}



pub struct TypeAddress<T: Asset> {
    // reference to the instances vector
    // RefCell allows interior mutability, since Rc by default doesn't allow the map to be mutable
    pub instances: Rc<RefCell<HashMap<Id, Rc<RefCell<T>>>>>
}

// HashMap instance implementation for Register<T>
// HashMap instance is the static hashmap you see at the top when you derive assets 
impl <T: Event, U: Asset> Register<T> for RefCell<HashMap<Id, Rc<RefCell<U>>>>
    where U: Receiver<T> {

    fn receivers(&self) -> Vec<Rc<RefCell<dyn Receiver<T>>>> {

        self.borrow().iter().map(|(_, instance)| {
            Rc::clone(instance) as Rc<RefCell<dyn Receiver<T>>>
            
        }).collect::<Vec<_>>()  
    }

    fn register_id(&self) -> Id {
        U::Metadata().id
    }
}

// implement Address for Type Addresses
impl <T: Event, U: Receiver<T> + Asset + 'static> Address<T> 
    for TypeAddress<U> {

        fn registers(self) -> Vec<Rc<dyn Register<T>>> {
            // as keyword to cast between SmartPointer<A> to SmartPointer<dyn B>
            vec![Rc::clone(&self.instances) as Rc<dyn Register<T>>]
        }
       
}

// implement Register for an asset instance
impl <T: Event, U: Receiver<T> + Asset> Register<T> for Rc<RefCell<U>> {
    fn receivers(&self) -> Vec<Rc<RefCell<dyn Receiver<T>>>> {
        vec![Rc::clone(self) as Rc<RefCell<dyn Receiver<T>>>]
    }
    
    fn register_id(&self) -> Id {
        self.borrow().metadata().id
    }
}


// Implement Address for an asset instance
impl <T: Event, U: Receiver<T> + Asset> Address<T> for Rc<RefCell<U>> {
    // takes ownership of the address
    fn registers(self) -> Vec<Rc<dyn Register<T>>> {
        vec![Rc::new(self)]
    }
}

pub struct AddressBuilder<T: Event> {
    registers: Vec<Rc<dyn Register<T>>>
}

impl <T: Event> AddressBuilder<T> {
    pub fn new() -> Self {
        AddressBuilder{registers: vec![]}
    }

    pub fn add(mut self, address: impl Address<T>) -> Self {
        self.registers.append(&mut address.registers());
        self
    }
}

impl <T: Event> Address<T> for AddressBuilder<T>{
    // takes ownership of the address
    fn registers(self) -> Vec<Rc<dyn Register<T>>> {
        self.registers
    }
}
