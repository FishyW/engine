
use crate::{prelude::*, router};


#[derive(Clone)]
struct ClickEvent {}


// makes __CLICK_EVENT_RECEIVERS local to the current thread
// this prevents usage of Arc<> and Mutex<>
thread_local!{
    static __CLICK_EVENT_RECEIVERS: StaticWeakMap<ClickEvent> = 
        std::cell::RefCell::new(HashMap::new());
}


impl Event for ClickEvent {

    fn register(item: &impl Receiver<Self>) {
         __CLICK_EVENT_RECEIVERS.with(|map| {
            let mut map = map.borrow_mut();
            map.insert(item.id(), item.weak_self());
         })
    }

    fn send(self, target: impl Address<Self>) 
        where Self:Sized {
        __CLICK_EVENT_RECEIVERS.with(|map| {
            router::send(self, target, &mut map.borrow_mut());
        })    
    }   

    fn broadcast(self) 
        where Self:Sized {
        __CLICK_EVENT_RECEIVERS.with(|map| {
            router::broadcast(self, &mut map.borrow_mut());
        })
    }
    
    // gets self instead of &self
    // so the router copy of the event can be implicitly dropped as well
    fn clear(self) {
        __CLICK_EVENT_RECEIVERS.with(|map| {
            map.borrow_mut()
                .clear()
        })
    }

}

