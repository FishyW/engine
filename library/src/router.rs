use ahash::HashMap;

use crate::prelude::*;

type EventMap<T> = HashMap<TypeId, Box<dyn EventRegister<T>>>;
type EventPropMap<T> = HashMap<TypeId, Box<dyn EventPropRegister<T>>>;


// given an address, call all receivers inside of that address
fn call_receiver<T>(event: T, receiver: &mut dyn Handler<T>)
    where  T:Event {
    receiver.handle(event);
}


// optimization idea is to use Cow, clone on write
// since cloning can be expensive, using Cow means, 
// data is only cloned when underlying data is mutated
// sends an event to the specified address
// note that this function fails silently if the address is already dropped
pub fn send<T: Event, U: Address<T>>(event: T, address: U) {
     
    address.receivers().into_iter().for_each(|mut receiver| {
        call_receiver(event.clone(), &mut receiver);
    })
}

// propagates an event, when no such PropReceivers can be found, this function fails silently
pub fn propagate<T: Event>(event: T, component_typeid: TypeId, instance_id: Id, 
    map: &mut EventPropMap<T>) {
        
        let Some(register) = map.get(&component_typeid) else {
            return;
        };

        let mut receivers = register.props();

        // should never fail since the component's instance id should
        //  refer to an object that exists
        let receiver = receivers.get_mut(&instance_id)
            .expect("Instance Id doesn't exist. Is the component valid?");

        call_receiver(event, receiver);
}

// called by the event system
pub fn broadcast<T: Event>(event: T, map: &mut EventMap<T>) {

    map.retain(|_, register| {
        register.receivers().into_iter().for_each(|mut recv| {
            call_receiver(event.clone(), &mut recv);
        });
        true
    });

}

