
use std::collections::VecDeque;

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

    fn prop_register(component_register: impl Register,
        register: impl EventPropRegister<Self> + 'static);

    // for the event router
    fn register_handler(event: Self, item: impl Handler<Self>);

    fn into_handle() -> HandleQueue<Self>;
}

pub trait Receiver<T: Event> {
    fn receive(&mut self, event: T);
}

pub trait Handle {
    fn handle(&self);
}

pub trait Handler<T>: 'static {
    fn handle(&mut self, asset: T);
}

pub struct HandleQueue<T: Event>  {
    pub queue: Rc<RefCell<VecDeque<(T, Box<dyn Handler<T>>)>>>,
}

impl <T: Event> Handle for HandleQueue<T> {
    fn handle(&self) {
        let (event, mut handler) = {
            // this is put into its own block, so that
            // the mutable borrow to the queue can go out of scope
            let mut queue = self.queue.borrow_mut();
            queue.pop_front()
                .expect("No more items inside of the deque")
        };
        
        handler.handle(event);
    }
}

impl <T: Event> Handler<T> for Rc<RefCell<dyn Receiver<T>>> {
    fn handle(&mut self, event: T) {
        self.borrow_mut().receive(event);
    }
}

impl <T: Event> Handler<T> for Rc<RefCell<dyn PropReceiver<T>>> {
    fn handle(&mut self, event: T) {
        self.borrow_mut().receive(event);
    }
}

// Propagation Event Receiver
pub trait PropReceiver<T: Event> {
    fn receive(&mut self, event: T);
}


// an address gives a vector of registers, which are usually assets
// that has been programmed to receive a certain event
pub trait Address<T: Event>: 'static {
    // needs RefCell since receive() borrows &mut self -> needs a mutable borrow
    // or receivers
    fn receivers(&self) -> Vec<Rc<RefCell<dyn Receiver<T>>>>;
}


// AddressBuilderId in case the only thing known about the address
// is the instance id of the object
// this is an internal API and it should never be used by the users
// struct AddressBuilderId<T: Asset> {
//     ids: Vec<Id>,
//     phantom: PhantomData<T>
// }

// impl <T: Asset> AddressBuilderId<T> {

//        // enter the type id
//     fn id<U: Asset>() -> AddressBuilderId<U> {
//         AddressBuilderId {ids: vec![], phantom: PhantomData}
//     }
//     fn add(mut self, id: Id) -> Self {
//         self.ids.push(id);
//         self
//     }
// }

// impl <T: Event, U: Object + Receiver<T>> Address<T> for AddressBuilderId<U>{
//     fn receivers<'a>(&'a self) -> Vec<Rc<RefCell<dyn Receiver<T> + 'a>>> {
//        let map = U::Address();
//        let map = map.map.borrow_mut();
//        self.ids.iter().map(|id| {
//             Rc::clone(map.get(&id).expect("Id not found!"))
//                 as Rc<RefCell<dyn Receiver<T>>>
//        }).collect()
//     }
// }

// creates an address by supplying a vector of addresses
pub struct AddressBuilder<T: Event> {
    addresses: Vec<Box<dyn Address<T>>>
}

impl < T: Event> AddressBuilder< T> {
    pub fn new() -> Self {
        AddressBuilder{addresses: vec![]}
    }

    pub fn add(mut self, address: impl Address<T> ) -> Self {
        self.addresses.push(Box::new(address));
        self
    }
}

impl < T: Event> Address<T> for AddressBuilder<T>{
    fn receivers(&self) -> Vec<Rc<RefCell<dyn Receiver<T>>>> {
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
pub trait EventPropRegister<T: Event> {
    fn props(& self) -> HashMap<Id, 
        Rc<RefCell<dyn PropReceiver<T>>>>;
}


impl <T: Event, U: PropReceiver<T> + Object> EventPropRegister<T> for
    InstanceMap<U> {
        fn props(&self) -> HashMap<Id, 
                Rc<RefCell<dyn PropReceiver<T>>>> {
            self.map.borrow().iter().map(|(&id, value)| {
                (id, Rc::clone(value) as Rc<RefCell<dyn PropReceiver<T>>>)
            }).collect()
        }
}

