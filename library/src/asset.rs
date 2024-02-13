mod event;
mod component;

use std::{cell::RefCell, rc::Rc};

use ahash::{HashMap, HashMapExt};
use crate::prelude::*;
pub use event::*;
pub use component::*;


// like Asset but it can be stored dynamically
// since Assets need to implement Default, which means that implementors
// Assets cannot be Unsized
pub trait Asset  {
    fn metadata(&self) -> InstanceMetadata;

    fn type_metadata(&self) -> TypeMetadata;
}


pub trait SizedAsset: Default + Asset {
    /// Note that this function may be slow
    /// since this function calls Self::default()
    /// if the object takes a long time to construct, 
    /// this will lead to performance issues
    /// An optimization idea is to implement Default 
    /// by retrieving from a cached default 
    #[allow(non_snake_case)]
    fn Metadata() -> TypeMetadata;
}

impl <T: Asset + Default> SizedAsset for T {
    #[allow(non_snake_case)]
    fn Metadata() -> TypeMetadata {
        Self::default().type_metadata()
    }
}

pub struct TypeMetadata {
    pub id: Id, 
    pub module_path: &'static str
}


#[derive(Default)]
pub struct InstanceMetadata {
    pub id: Id
}

// Object is Sized
pub trait Object: SizedAsset {
    // used to register an object
    fn register(asset: Self) -> std::rc::Rc<std::cell::RefCell<Self>>;

    // to get all instances of an object use Address()
   #[allow(non_snake_case)]
   // Self:Sized means you can't get the address of a dyn trait object
   fn Address() -> InstanceMap<Self>
       where Self:Sized;
}

pub trait UnsizedObject: Asset {}

impl <T: Object> UnsizedObject for T {}


// map of instances, used for components and objects
pub struct InstanceMap<T>  {
    pub map: Rc<RefCell<HashMap<Id, Rc<RefCell<T>>>>>,
    // id is the type id
    pub id: Id
}

impl <T> InstanceMap<T> {
    pub fn new(type_id: Id) -> InstanceMap<T> {
        InstanceMap{map: Rc::new(RefCell::new(HashMap::new())), id: type_id}
    }
}


// Registers are any assets or asset types that can be registered
pub trait Register {
    // get the id used to register
    fn register_id(&self) -> Id;
}


// Implement Address for an asset instance (Rc<RefCell<U>>)
impl <T: Event, U: Receiver<T> + Asset> Address<T> for Rc<RefCell<U>> {
    // takes ownership of the address
    fn receivers<'a>(&'a self) -> Vec<Rc<RefCell<dyn Receiver<T> + 'a>>> {
        vec![Rc::clone(&self) as Rc<RefCell<dyn Receiver<T>>>]
    }
}

// implement Address for an instance map
impl <T: Event, U: Receiver<T> + Object> Address<T> 
    for InstanceMap<U> {
        fn receivers<'a>(&'a self) -> Vec<Rc<RefCell<dyn Receiver<T> + 'a>>> {
            // as keyword to cast between SmartPointer<A> to SmartPointer<dyn B>
            self.map.borrow().iter().map(|(_, instance)| {
                Rc::clone(instance) as Rc<RefCell<dyn Receiver<T>>>
                
            }).collect()
        }
}

impl <T> Register for InstanceMap<T> {
    fn register_id(&self) -> Id {
        self.id
    }
}

struct Scene {}