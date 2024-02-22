
use crate::prelude::*;

#[derive(Clone)]
pub struct ClickEvent;

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

    static __CLICK_EVENT_HANDLERS: std::cell::RefCell<
    std::rc::Rc<std::cell::RefCell<
    std::collections::VecDeque<
    (ClickEvent, Box<dyn Handler<ClickEvent>>) 
    >>>> = Default::default()
}

impl Event for ClickEvent {
    
    // method is called when users use the prop receivers macro
    fn prop_register(component_register: impl Register,
        register: impl EventPropRegister<Self> + 'static) {
        __CLICK_EVENT_PROP_REGISTER.with(|map| {
            let mut map = map.borrow_mut();
            let id = component_register.register_id();
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


    // for the router code
    fn register_handler(event: Self, item: impl Handler<Self>) {
        __CLICK_EVENT_HANDLERS.with(|queue| {
            let queue = queue.borrow_mut();
            queue.borrow_mut()
                .push_back((event, Box::new(item)));
         })
    }

    fn into_handle() -> HandleQueue<Self> {
        let queue = __CLICK_EVENT_HANDLERS.with(|queue| {
            let queue = queue.borrow_mut();
            std::rc::Rc::clone(&queue)
        });
        HandleQueue {queue}
    }
}



