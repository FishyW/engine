
use std::{cell::RefCell, rc::Rc};

use ahash::HashMap;

use crate::prelude::*;

type EventMap<T> = HashMap<TypeId, Box<dyn EventRegister<T>>>;
type EventPropMap<T> = HashMap<TypeId, Box<dyn EventPropRegister<T>>>;

#[derive(Default)]
struct Router {
    // the stack stores a list of events that will be sent
    stack: Vec<(Id, Box<dyn RouterRegister>)>,

    current_handler: EventAsset
}

// Router is a static (singleton) variable
thread_local! {
    static ROUTER: RefCell<Router> = RefCell::new(Router::default());
}

const LOOP_MAX_ITERATIONS: u32 = 100000;

// start the event loop
// this is called when an event is received from JavaScript
// the event loop executes all events via a DFS search through the event dependency graph
pub fn start() {
    let mut event_stack = Vec::new();
    
    for i in 1..=LOOP_MAX_ITERATIONS {
        if i == LOOP_MAX_ITERATIONS {
            panic!("Event Loop reaches maximum number of iterations. \
            Are you not in an infinite loop?");
        }

        // populate the event stack
        ROUTER.with(|router| {
            let mut router = router.borrow_mut();
            let mut stack = std::mem::take(&mut router.stack);
            loop {
                let Some(value) = stack.pop() else {
                    break;
                };
                event_stack.push(value);
            }
        });

     
        let receiver = event_stack.pop();

        // if no more events left inside of the event queue 
        // the event loop is over
        let Some((id, receiver)) = receiver else {
            break;
        };
        receiver.call_receivers(id);
    }
}

// calls the interceptors and event actions, then calls the receiver
pub fn call_receiver<T: Event>(event: T, source: EventAsset, 
        receiver: Rc<RefCell<dyn Receiver<T>>>) {

    let mut receiver = receiver.borrow_mut();
    
    let event = Incoming::new(
            event, source, receiver.into_event_asset());

    receiver.receive(event);
}

// register the event into the event queue
fn register_event<T: Event>(event: T, receiver: Rc<RefCell<dyn Receiver<T>>>) {
    let source = ROUTER.with(|router| {
        router.borrow().current_handler.clone()
    });

    let id = Id::default();
    T::register_event(id, event, source, receiver);
    ROUTER.with(|router| {
            let mut router = router.borrow_mut();
            router.stack.push((id, Box::new(T::into_register())));
    });
}


// optimization idea is to use Cow, clone on write
// since cloning can be expensive, using Cow means, 
// data is only cloned when underlying data is mutated
// sends an event to the specified address
// note that this function fails silently if the address is already dropped
pub fn send<T: Event, U: Address<T>>(event: T, address: U) {
    address.receivers().into_iter().for_each(|receiver| {
        register_event(event.clone(), receiver);
    })
}

// propagates an event, when no such PropReceivers can be found, this function fails silently
pub fn propagate<T: Event>(event: T, component_typeid: TypeId, instance_id: Id, 
    map: &mut EventPropMap<T>) {
        
        let Some(register) = map.get(&component_typeid) else {
            return;
        };

        let receivers = register.receivers();

        // should never fail since the component's instance id should
        //  refer to an object that exists
        let receiver = receivers.get(&instance_id)
            .expect("Instance Id doesn't exist. Is the component valid?");

        register_event(event, Rc::clone(receiver));
}

// called by the event system
pub fn broadcast<T: Event>(event: T, map: &mut EventMap<T>) {
    map.retain(|_, register| {
        register.receivers().into_iter().for_each(|recv| {
            register_event(event.clone(), recv);
        });
        true
    });

}

