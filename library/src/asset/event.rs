
use std::ops::{Deref, DerefMut};

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

    fn prop_register(register: impl EventPropRegister<Self> + 'static);

    // for the event router
    fn register_event(id: Id,
        event: Self, source: EventAsset, item: Rc<RefCell<dyn Receiver<Self>>>);

    fn into_register() -> RouterMap<Self>;
}

// incoming "smart pointer"
#[derive(Clone)]
pub struct Incoming<T: Event> {
    pub target: EventAsset,
    pub source: EventAsset,
    event: T
}

impl <T: Event> Incoming<T> {
    pub fn new(event: T,  source: EventAsset, target: EventAsset) -> Incoming<T> {
        Incoming{target, source, event}
    }
}


pub trait IntoEvent<T: Event> {
    fn into_event(self) -> T;
}

impl <T: Event> IntoEvent<T> for Incoming<T> {
    // takes an event out of the Incoming<T>
    fn into_event(self) -> T {
        self.event
    }
}

impl <T: Event> IntoEvent<T> for T {
    fn into_event(self) -> T {
        self
    }
}

impl <T: Event> Deref for Incoming<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.event
    }
}

impl <T: Event> DerefMut for Incoming<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.event
    }
}

pub trait Receiver<T: Event>: Asset {
    fn receive(&mut self, event: Incoming<T>);
}

pub trait Interceptor<T: Event> {
    fn intercept(&mut self, event: T) -> Option<T>;
}


#[derive(Clone)]
pub struct EventAsset {
    pub metadata: InstanceMetadata,
    pub type_metadata: TypeMetadata,
}

impl std::fmt::Debug for EventAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.type_metadata.module_path)
    }
}

impl Default for EventAsset {
    fn default() -> Self {
        EventAsset {
            metadata: InstanceMetadata {id: Id::empty()},
            type_metadata: TypeMetadata {id: TypeId::empty(), 
                module_path: ""}
        }
    }
}

pub trait RouterRegister {
    fn call_receivers(&self, event_id: Id);
}

pub struct RouterMap<T: Event>  {
    pub map: Rc<RefCell<HashMap<Id, (T, EventAsset, Rc<RefCell<dyn Receiver<T>>>)>>>,
}

impl <T: Event> Default for RouterMap<T> {
    fn default() -> Self {
        RouterMap {map: Rc::new(RefCell::new(HashMap::new()))}
    }
}

impl <T: Event> RouterRegister for RouterMap<T> {
    fn call_receivers(&self, event_id: Id) {
        let (event, source, receiver) = {
            // this is put into its own block, so that
            // the mutable borrow to the queue can go out of scope
            // before handler.handle() is called
            let mut map = self.map.borrow_mut();
            map.remove(&event_id)
                .expect("Id does not exist in the router map!")
        };
        
        crate::router::call_receiver(event, source, receiver);
    }
}

// Propagation Event Receiver
pub trait PropReceiver<T: Event, U: Component>: Include<U> {
    fn receive(&mut self, event: Incoming<T>);
}


// an address gives a vector of registers, which are usually assets
// that has been programmed to receive a certain event
pub trait Address<T: Event>: 'static {
    // needs RefCell since receive() borrows &mut self -> needs a mutable borrow
    // or receivers
    fn receivers(&self) -> Vec<Rc<RefCell<dyn Receiver<T>>>>;
}


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

impl <T: Event> Address<T> for AddressBuilder<T>{
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
pub trait EventPropRegister<T: Event>: Register {
    fn receivers(&self) -> HashMap<Id, 
        Rc<RefCell<dyn Receiver<T>>>>;
}


// helper struct to provide the blanket implementation for Prop Addresses
struct PropReceiverInstance<T: Component, U: Include<T>>{
    object: Rc<RefCell<U>>,
    phantom: PhantomData<T>
}

impl <T: Component, U: Include<T>> Asset for PropReceiverInstance<T, U> {
    fn metadata(&self) -> InstanceMetadata {
        self.object.borrow().metadata()
    }
    fn type_metadata(&self) -> TypeMetadata {
        self.object.borrow().type_metadata()
    }
}

impl <T: Event, U: Component, V: PropReceiver<T, U>> Receiver<T> 
    for PropReceiverInstance<U, V> {
    fn receive(&mut self, event: Incoming<T>) {
        self.object.borrow_mut().receive(event);
    }
}


impl <T: Component, U: Include<T>> Register for PropAddress<T, U> {
    fn register_id(&self) -> TypeId {
        T::Address().id
    }
}

// blanket implementation for Object::PropAddress()
impl <T: Event, U: Component, V: PropReceiver<T, U>> EventPropRegister<T> for
    PropAddress<U, V> {
        fn receivers(&self) -> HashMap<Id, 
                Rc<RefCell<dyn Receiver<T>>>> {
            self.map.map.borrow().iter().map(|(&id, object)| {
                let object = Rc::clone(object);
                let instance = 
                    PropReceiverInstance{object, 
                    phantom: PhantomData::default()};
                
                let instance = Rc::new(RefCell::new(instance)) 
                    as Rc<RefCell<dyn Receiver<T>>>;

                (id, instance)
            }).collect()
        }
}

