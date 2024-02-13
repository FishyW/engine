use ahash::HashMap;

use crate::prelude::*;

use std::{cell::RefCell, rc::Rc};

type EventMap<T> = HashMap<Id, Box<dyn EventRegister<T>>>;

// given an address, call all receivers inside of that address
fn call_receiver<'a, T>(event: T, receiver: Rc<RefCell<dyn Receiver<T> + 'a>>)
    where  T:Event {
    receiver.borrow_mut().receive(event);
}


// optimization idea is to use Cow, clone on write
// since cloning can be expensive, using Cow means, 
// data is only cloned when underlying data is mutated
// sends an event to the specified address
// note that this function fails silently if the address is already dropped
pub fn send<T, U>(event: T, address: U)
    where T: Event, U: Address<T> {
        address.receivers().into_iter().for_each(|receiver| {
            call_receiver(event.clone(), receiver);
        })
}

// called by the event system
pub fn broadcast<T: Event>(event: T, map: &mut EventMap<T>) {

    map.retain(|_, register| {
        register.receivers().into_iter().for_each(|recv| {
            call_receiver(event.clone(), recv);
        });
        true
    });

}

