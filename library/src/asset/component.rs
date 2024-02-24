
use super::*;

pub trait Component: SizedAsset  {
    // register an object, this is called when implementing the Include<> trait
    fn register<T: Include<Self>  + 'static>(component_id: TypeId, object: InstanceMap<T>);

    // to get all instances of an object use Address()
    #[allow(non_snake_case)]
    fn Address() -> ComponentMap<Self>;


    fn propagate<T: Event, U: IntoEvent<T>>(&self, event: U) {
        event.into_event().propagate(self);
    }

    fn parent(&self) -> &dyn Asset;


}


pub trait IncludeUnsized<T: Component>: UnsizedObject {
    // takes a component out of its parent
    fn take(&mut self) -> T;

    fn get<'a>(&'a mut self) -> &'a mut T;

    // puts a component back
    fn put(&mut self, component: T);

}

pub trait GetComponent<U: ?Sized> {
    fn get<T: Component>(&mut self) -> &mut T
        where U: IncludeUnsized<T>;
}




pub trait Include<T: Component>: Object + IncludeUnsized<T> {
    #[allow(non_snake_case)]
    // prop addresses are used internally, don't use this
    fn PropAddress() -> PropAddress<T, Self>;
}


// map of instances, used for components and objects
// a component instance keeps track of its parent from the component map
pub struct ComponentMap<T: Component> {
    pub map: Rc<RefCell<HashMap<TypeId, Box<dyn IncludeRegister<T>>>>>,
    // id is the type id
    pub id: TypeId
}

impl <T: Component> ComponentMap<T> {
    pub fn new(type_id: TypeId) -> ComponentMap<T> {
        ComponentMap{map: Rc::new(RefCell::new(HashMap::new())), 
            id: type_id}
    }
}


pub trait IncludeRegister<T: Component>: Register {
    fn registers(&self) -> HashMap<Id, Rc<RefCell<dyn IncludeUnsized<T>>>>;
}


// implement include register for instance maps
impl <T: Component, U: Include<T> + 'static> IncludeRegister<T> for InstanceMap<U> {
    fn registers(&self) -> HashMap<Id, Rc<RefCell<dyn IncludeUnsized<T>>>> {
        self.map.borrow_mut().iter().map(|(&key, val)| {
            (key, Rc::clone(val) as Rc<RefCell<dyn IncludeUnsized<T>>>)
        }).collect()
    }
}


impl <U: Component> Asset for Rc<RefCell<dyn IncludeUnsized<U>>> {
    fn metadata(&self) -> InstanceMetadata {
        self.borrow_mut().get().metadata()
    }

    fn type_metadata(&self) -> TypeMetadata {
        self.borrow_mut().get().type_metadata()
    }
}

// Implement receiver for Component Receivers
impl <T: Event, U: Receiver<T> + Component> Receiver<T> for Rc<RefCell<dyn IncludeUnsized<U>>>
    {
    fn receive(&mut self, event: Incoming<T>) {
        let mut component = {
            // parent's mutable borrow ends
            // component temporarily belongs to the router
            self.borrow_mut().take()
        };
        component.receive(event);
        self.borrow_mut().put(component);
        
    }
}

// convert component map into addresses
impl <T: Event, U: Component + Receiver<T>> Address<T> for 
    ComponentMap<U> {
    fn receivers(&self) -> Vec<Rc<RefCell<dyn Receiver<T>>>> {
        
        self.map.borrow().values().flat_map(|register| {
            let objects = register.registers();
            objects.into_iter().map(|(_, obj)| {
                Rc::new(RefCell::new(obj)) as Rc<RefCell<dyn Receiver<T>>>
            })
        }).collect()
    }
}

impl <T: Component> Register for ComponentMap<T> {
    fn register_id(&self) -> TypeId {
        self.id
    }
}


