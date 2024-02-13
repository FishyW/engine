

use crate::{prelude::*, router};

#[derive(Clone)]
pub struct ClickEvent;


use ahash::HashMapExt;

// makes __CLICK_EVENT_RECEIVERS local to the current thread
// this prevents usage of Arc<> and Mutex<>
thread_local!{
    // note this long type is done on purpose due to macro hygiene concerns
    static __CLICK_EVENT_ADDRESSES: std::cell::RefCell<ahash::HashMap<Id, 
        Box<dyn Register<ClickEvent>>>> = 
        std::cell::RefCell::new(ahash::HashMap::new());
}

impl Event for ClickEvent {

    fn register(register: impl Register<Self> + 'static) {
         __CLICK_EVENT_ADDRESSES.with(|map| {
            let mut map = map.borrow_mut();
            let id = register.register_id();
            map.insert(id, Box::new(register));
         })
    }

    fn send(self, target: impl Address<Self>) 
        where Self:Sized {
        __CLICK_EVENT_ADDRESSES.with(|_| {
            router::send(self, target);
        })    
    }   

    fn broadcast(self) 
        where Self:Sized {
        __CLICK_EVENT_ADDRESSES.with(|map| {
            router::broadcast(self, &mut map.borrow_mut());
        })
    }

    // gets self instead of &self
    // so the router copy of the event can be implicitly dropped as well
    fn clear(self) {
        __CLICK_EVENT_ADDRESSES.with(|map| {
            map.borrow_mut()
                .clear()
        })
    }

}


