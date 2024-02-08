mod event;
mod address;

use std::rc::Weak;

pub use address::*;
use id::Id;

use crate::router;

// note that Self:Sized means that you can't call this method on a dynamic object

pub trait Asset: 'static {
    fn id(&self) -> Id;
    fn type_id(&self) -> Id;

    fn weak_self(&self) -> Weak<Self>
        where Self:Sized;


}

pub trait Object: Asset {
    // given an instance id, get the instance with that address
    fn get_instance(id: Id) -> Self;
}

pub trait Component: Asset {}

pub trait Include<T: Component>: Object {}

// 'static means event must be implemented for an owned type
pub trait Event: Clone + 'static {

    fn send(self, target: impl Address<Self>) 
        where Self:Sized;

    fn broadcast(self) 
        where Self:Sized;

    // register an address to this receiver
    fn register(item: &impl Receiver<Self>) 
        where Self:Sized;

    // cleanup function
    fn clear(self);

}

pub trait Receiver<T: Event>: Asset {
    fn receive(&self, event: T);
}

