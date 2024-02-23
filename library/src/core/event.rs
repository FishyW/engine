
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

    // stores a map of type id that maps to the component map
    static __CLICK_EVENT_PROP_REGISTER: std::cell::RefCell<ahash::HashMap<TypeId, 
        Box<dyn EventPropRegister<ClickEvent>>>> = 
        std::cell::RefCell::new(ahash::HashMap::new());

    static __CLICK_EVENT_ROUTER_MAP: RouterMap<ClickEvent> = Default::default()
}

impl Event for ClickEvent {
    
    // registers a propagation receiver
    // method is called when users use the prop receivers macro
    fn prop_register(register: impl EventPropRegister<Self> + 'static) {
        __CLICK_EVENT_PROP_REGISTER.with(|map| {
            let id = register.register_id();
            map.borrow_mut().insert(id, Box::new(register));
        })
    }

    // registers a receiver
    // method is called when users use the receivers macro
    fn register(register: impl EventRegister<Self> + 'static) {
         __CLICK_EVENT_REGISTER.with(|map| {
            let id = register.register_id();
            map.borrow_mut().insert(id, Box::new(register));
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


    // for the router code
    fn register_event(
        id: Id,
        event: Self, 
        source: EventAsset,
        recv: std::rc::Rc<std::cell::RefCell<dyn Receiver<Self>>>) {
        __CLICK_EVENT_ROUTER_MAP.with(|map| {
            let mut map = map.map.borrow_mut();
            map.insert(id, (event, source, recv));
         })
    }

    fn into_register() -> RouterMap<Self> {
        let map = __CLICK_EVENT_ROUTER_MAP.with(|map| {
            std::rc::Rc::clone(&map.map)
        });
        RouterMap {map}
    }

}



