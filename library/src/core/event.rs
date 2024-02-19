
use crate::{prelude::*, router};

#[derive(Clone)]
pub struct ClickEvent;


use ahash::HashMapExt;


// makes __CLICK_EVENT_RECEIVERS local to the current thread
// this prevents usage of Arc<> and Mutex<>
thread_local!{
    // note this long type is done on purpose due to macro hygiene concerns
    static __CLICK_EVENT_REGISTER: std::cell::RefCell<ahash::HashMap<TypeId, 
        Box<dyn EventRegister<ClickEvent>>>> = 
        std::cell::RefCell::new(ahash::HashMap::new());

    static __CLICK_EVENT_PROP_REGISTER: std::cell::RefCell<ahash::HashMap<TypeId, 
        Box<dyn EventPropRegister<ClickEvent>>>> = 
        std::cell::RefCell::new(ahash::HashMap::new());
}

impl Event for ClickEvent {
    
    // method is called when users use the prop receivers macro
    fn prop_register(register: impl EventPropRegister<Self> + 'static) {
        __CLICK_EVENT_PROP_REGISTER.with(|map| {
            let mut map = map.borrow_mut();
            let id = register.register_id();
            map.insert(id, Box::new(register));
        })
    }

    // method is called when users use the receivers macro
    fn register(register: impl EventRegister<Self> + 'static) {
         __CLICK_EVENT_REGISTER.with(|map| {
            let mut map = map.borrow_mut();
            let id = register.register_id();
            map.insert(id, Box::new(register));
         })
    }

    fn send(self, target: impl Address<Self>) 
        where Self:Sized {
        __CLICK_EVENT_REGISTER.with(|_| {
            router::send(self, target);
        })    
    }   

    fn broadcast(self) 
        where Self:Sized {
        __CLICK_EVENT_REGISTER.with(|map| {
            router::broadcast(self, &mut map.borrow_mut());
        })
    }

    fn propagate(self, component: &impl Component) {
        __CLICK_EVENT_PROP_REGISTER.with(|map| {
            router::propagate(self, 
                component.type_metadata().id,
                component.component_metadata().parent_id,
                &mut map.borrow_mut());
        })
    }

    // gets self instead of &self
    // so the router copy of the event can be implicitly dropped as well
    fn clear(self) {
        __CLICK_EVENT_REGISTER.with(|map| {
            map.borrow_mut()
                .clear()
        })
    }

}


