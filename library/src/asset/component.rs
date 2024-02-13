
use super::*;

pub trait Component: SizedAsset  {
    // register an object, this is called when implementing the With<> trait
    fn register<T: Include<Self>  + 'static>(component_id: Id, object: InstanceMap<T>);

    // to get all instances of an object use Address()
    #[allow(non_snake_case)]
    fn Address() -> ComponentMap<Self>;
    
}

pub trait Include<T: Component>: UnsizedObject {
    fn get<'a>(&'a self) -> &'a mut T;
}

// map of instances, used for components and objects
pub struct ComponentMap<T: Component> {
    pub map: Rc<RefCell<HashMap<Id, Box<dyn IncludeRegister<T>>>>>,
    // id is the type id
    pub id: Id
}

impl <T: Component> ComponentMap<T> {
    pub fn new(type_id: Id) -> ComponentMap<T> {
        ComponentMap{map: Rc::new(RefCell::new(HashMap::new())), 
            id: type_id}
    }
}

pub trait IncludeRegister<T: Component> {
    fn include_register_id(&self) -> Id;

    fn include_objects(&self) -> Vec<Rc<RefCell<dyn Include<T>>>>;
}

impl <T: Component, U: Include<T> + 'static> IncludeRegister<T> for InstanceMap<U> {
    fn include_objects(&self) -> Vec<Rc<RefCell<dyn Include<T>>>> {
        self.map.borrow_mut().values().into_iter().map(|val| {
            Rc::clone(val) as Rc<RefCell<dyn Include<T>>>
        }).collect()
    }

    fn include_register_id(&self) -> Id {
        self.id
    }
}


impl <T: Event, U: Receiver<T> + Component> Receiver<T> for Rc<RefCell<dyn Include<U>>> {
    fn receive(&mut self, event: T) {
        self.borrow_mut()
            .get().receive(event);
    }
}

impl <T: Event, U: Component + Receiver<T>> Address<T> for 
    ComponentMap<U> {
    fn receivers<'a>(&'a self) -> Vec<Rc<RefCell<dyn Receiver<T> + 'a>>> {
        let mut receivers = vec![];
        self.map.borrow().values().for_each(|register| {
            let objects = register.include_objects();
            objects.into_iter().for_each(|obj| {
                receivers.push(
                    Rc::new(RefCell::new(obj)) as Rc<RefCell<dyn Receiver<T>>>
                )
            })
        });

        receivers
    }
}


