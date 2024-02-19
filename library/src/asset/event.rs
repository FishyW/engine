use std::{collections::HashMap, marker::PhantomData};

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
    };

}



// 'static means event must be implemented for an owned type
pub trait Event: Clone + 'static {

    fn send(self, target: impl Address<Self>);

    fn broadcast(self);

    fn propagate(self, component: &impl Component);

    // register an address to this event
    // this iterates through all registers specified by the address
    // and registers each register to the event
    fn register(item: impl EventRegister<Self> + 'static);


    // cleanup function when scene changes
    fn clear(self);

    fn prop_register(item: impl EventPropRegister<Self> + 'static);
}

pub struct PropEvent<T: Event> {
    pub event: T
}


impl <T: Event> From<T> for PropEvent<T> {
    fn from(event: T) -> Self {
        PropEvent {event}
    }
}

pub trait Handler<T> {
    fn handle(&mut self, asset: T);
}

pub trait Receiver<T: Event> {
    fn receive(&mut self, event: T);
}

impl <'a, T: Event> Handler<T> for Rc<RefCell<dyn Receiver<T> + 'a>> {
    fn handle(&mut self, event: T) {
        self.borrow_mut().receive(event);
    }
}

impl <'a, T: Event> Handler<T> for Rc<RefCell<dyn PropReceiver<T> + 'a>> {
    fn handle(&mut self, event: T) {
        self.borrow_mut().receive(event.into());
    }
}

// Propagation Event Receiver
pub trait PropReceiver<T: Event> {
    fn receive(&mut self, event: PropEvent<T>);
}


// an address gives a vector of registers, which are usually assets
// that has been programmed to receive a certain event
pub trait Address<T: Event> {
    // needs RefCell since receive() borrows &mut self -> needs a mutable borrow
    // or receivers
    fn receivers<'a>(&'a self) -> Vec<Rc<RefCell<dyn Receiver<T> + 'a>>>;
}


// AddressBuilderId in case the type of the address isn't known
pub struct AddressBuilderId<T: Asset> {
    ids: Vec<Id>,
    phantom: PhantomData<T>
}

impl <T: Asset> AddressBuilderId<T> {
    pub fn add(mut self, id: Id) -> Self {
        self.ids.push(id);
        self
    }
}

impl <T: Event, U: Object + Receiver<T>> Address<T> for AddressBuilderId<U>{
    fn receivers<'a>(&'a self) -> Vec<Rc<RefCell<dyn Receiver<T> + 'a>>> {
       let map = U::Address();
       let map = map.map.borrow_mut();
       self.ids.iter().map(|id| {
            Rc::clone(map.get(&id).expect("Id not found!"))
                as Rc<RefCell<dyn Receiver<T>>>
       }).collect()
    }
}

// creates an address by supplying a vector of addresses
pub struct AddressBuilder<'a, T: Event> {
    addresses: Vec<Box<dyn Address<T> + 'a>>
}

impl <'a, T: Event> AddressBuilder<'a, T> {
    pub fn new() -> Self {
        AddressBuilder{addresses: vec![]}
    }

    // enter the type id
    pub fn id<U: Asset>() -> AddressBuilderId<U> {
        AddressBuilderId {ids: vec![], phantom: PhantomData}
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

pub trait EventRegister<T: Event>: Register + Address<T> {}

impl <T: Event, U: Register + Address<T>> EventRegister<T> for U {}


// similar to register except it returns a HashMap of <Id, Rc<RefCell<dyn PropReceiver<T>>>
// Id is the instance Id and Prop are all objects that has a propagation receiver
pub trait EventPropRegister<T: Event>: Register {
    fn props<'a>(&'a self) -> HashMap<Id, 
        Rc<RefCell<dyn PropReceiver<T> + 'a>>>;
}


impl <T: Event, U: PropReceiver<T> + Object> EventPropRegister<T> for
    InstanceMap<U> {
        fn props<'a>(&'a self) -> HashMap<Id, 
                Rc<RefCell<dyn PropReceiver<T> + 'a>>> {
            self.map.borrow().iter().map(|(&id, value)| {
                (id, Rc::clone(value) as Rc<RefCell<dyn PropReceiver<T>>>)
            }).collect()
        }
}

