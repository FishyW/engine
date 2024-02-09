pub mod event;
mod address;

use std::{cell::RefCell, rc::{Rc, Weak}};

pub use address::*;
use ahash::HashMap;
pub use crate::prelude::*;



pub trait Asset: 'static + Default {
    fn metadata(&self) -> InstanceMetadata;

    fn type_metadata(&self) -> TypeMetadata {
        Self::Metadata()
    }

    #[allow(non_snake_case)]
    fn Metadata() -> TypeMetadata;

    // to get all instances of an object use Address()
    #[allow(non_snake_case)]
    // Self:Sized means you can't get the address of a dyn trait object
    fn Address() -> TypeAddress<Self>
        where Self:Sized;


    // gets the Rc reference of itself
    // do not ever store this value inside of a static variable
    // or else it won't ever be cleaned up
    fn to_ref(&self) -> Rc<Self>
        where Self:Sized;
        
}

pub struct TypeMetadata {
    pub id: Id, 
    pub module_path: &'static str
}

pub struct InstanceMetadata {
    pub id: Id
}

pub struct TypeAddress<T: Asset> {
    // reference to the instances vector
    pub instances: Rc<HashMap<Id, Rc<T>>>
}

// implement Register for Type Addresses
impl <T: Event, U: Receiver<T> + Asset + 'static> Register<T> 
    for TypeAddress<U> {
        // receiver is never stored
        fn receiver(&self) -> Vec<Rc<dyn Receiver<T>>> {
            self.instances.iter().map(|(_, instance)| {
                instance.to_ref() as Rc<dyn Receiver<T>>
            }).collect::<Vec<_>>()
        }
        
        fn register_id(&self) -> Id {
            U::Metadata().id
        }

        fn to_ref(&self) -> Rc<Self>
                where Self:Sized {
            // Rc::clone is cheap since only a reference is being copied
            Rc::new(TypeAddress {
                instances: Rc::clone(&self.instances)
            })
        }
}

// implement Register for an asset instance
impl <T: Event, U: Receiver<T> + Asset> Register<T> for U {
    fn receiver(&self) -> Vec<Rc<dyn Receiver<T>>> {
        vec![self.to_ref()]
    }
    
    fn register_id(&self) -> Id {
        self.metadata().id
    }

    fn to_ref(&self) -> Rc<Self>
            where Self:Sized {
        self.to_ref()
    }
}

pub trait Object: Asset {}

pub trait Component: Asset {}

pub trait Include<T: Component>: Object {}

// 'static means event must be implemented for an owned type
pub trait Event: Clone + 'static {

    fn send(self, target: impl Address<Self>) 
        where Self:Sized;

    fn broadcast(self) 
        where Self:Sized;

    // register an address to this receiver
    fn register(item: &impl Register<Self>);

    // cleanup function
    fn clear(self);

}

pub trait Receiver<T: Event>: 'static{
    fn receive(&self, event: T);
}

