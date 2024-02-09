use ahash::HashMap;

use crate::prelude::*;

use std::rc::{Rc, Weak};
type EventWeakMap<T> = HashMap<Id, Weak<dyn Register<T>>>;


// given an address, call all receivers inside of that address
fn call_register<T>(event: T, register: Rc<dyn Register<T>>)
    where  T:Event {
    let recv = register.receiver();

    recv.iter()
        .for_each(|recv| recv.receive(event.clone()));
}


// optimization idea is to use Cow, clone on write
// since cloning can be expensive, using Cow means, 
// data is only cloned when underlying data is mutated
// sends an event to the specified address
// note that this function fails silently if the address is already dropped
pub fn send<T, U>(event: T, address: U)
    where T: Event, U: Address<T> {
        address.registers().into_iter().for_each(|register| {
            call_register(event.clone(), register);
        })
}

// called by the event system
pub fn broadcast<T: Event>(event: T, map: &mut EventWeakMap<T>) {
    
    map.retain(|_, register| {
        // return false means remove this item from the map
        // recv.upgrade() becomes None if the underlying data is dropped
        // if that's the case clean up the data, by removing the data from the map
        let Some(register) = register.upgrade() else {
            return false;
        }; 


        call_register(event.clone(), register);
        true
    });

}

