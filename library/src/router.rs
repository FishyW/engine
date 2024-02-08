use crate::prelude::*;

use std::rc::{Rc, Weak};
type EventWeakMap<T> = HashMap<Id, Weak<dyn Receiver<T>>>;

use super::*;

fn call_receiver<T: Event>(event: T, recv: Rc<dyn Receiver<T>>) {
    recv.receive(event);
}


pub fn send<T, U>(event: T, address: U, map: &mut EventWeakMap<T>)
    where T: Event, U: Address<T> {
        
}

// called by the event system
pub fn broadcast<T: Event>(event: T, map: &mut EventWeakMap<T>) {
    
    map.retain(|_, recv| {
        // return false means remove this item from the map
        // recv.upgrade() becomes None if the underlying data is dropped
        // if that's the case clean up the data, by removing the data from the map
        let Some(recv) = recv.upgrade() else {
            return false;
        }; 
        call_receiver(event.clone(), recv);
        true
    });

}

