use std::{collections::HashMap, rc::Weak};

use super::*;

// map of instances, used for components and objects
pub struct ComponentMap<T: UnsizedObject + ?Sized> {
    pub map: Rc<RefCell<HashMap<Id, Weak<RefCell<T>>>>>,
    // id is the type id
    pub id: Id
}



pub trait Component: SizedAsset  {
    // register an object, this is called when implementing the With<> trait
    fn register(object: impl Include<Self> + 'static);

    // to get all instances of an object use Address()
    #[allow(non_snake_case)]
    fn Address<'a>() -> ComponentMap<dyn Include<Self>>;
    
}

pub trait Include<T: Component>: UnsizedObject {
    fn get<'a>(&'a self) -> &'a mut T;
}


// used for the blanket implementation
struct ComponentReceiver<T: Component>  {
    object: Rc<RefCell<dyn Include<T>>>
}

impl <'a, T: Component> ComponentReceiver< T> {
    fn new(object: Rc<RefCell<dyn Include<T>>>) -> ComponentReceiver<T> {
        ComponentReceiver{object}
    }
}

impl <'a, T: Event, U: Receiver<T> + Component> Receiver<T> for ComponentReceiver< U>{
    fn receive(&mut self, event: T) {
        self.object.borrow_mut()
            .get().receive(event);
    }
}


impl <T: Event, U: Component + Receiver<T>> Address<T> for 
    ComponentMap<dyn Include<U>> {
    fn receivers<'a>(&'a self) -> Vec<Rc<RefCell<dyn Receiver<T> + 'a>>> {
        let mut receivers = vec![];
        self.map.borrow_mut().retain(|_, object| {
            let Some(obj) = object.upgrade() else {
                return false;
            };
            let oc_receiver = ComponentReceiver::new(obj);
           receivers.push(
                Rc::new(RefCell::new(oc_receiver)) as Rc<RefCell<dyn Receiver<T>>>
            );
           true
        });

        receivers
    }
}


