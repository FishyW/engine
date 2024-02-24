mod event;
mod component;

use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use ahash::{HashMap, HashMapExt};
use crate::prelude::*;
pub use event::*;
pub use component::*;

// like Asset but it can be stored dynamically
// since Assets need to implement Default, which means that implementors
// Assets cannot be Unsized
pub trait Asset: 'static {
    fn metadata(&self) -> InstanceMetadata;

    fn type_metadata(&self) -> TypeMetadata;

    // EventAsset is a wrapper type for the two metadata above
    fn into_event_asset(&self) -> EventAsset {
        EventAsset {
            metadata: self.metadata(),
            type_metadata: self.type_metadata()
        }
    }
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


pub struct PropAddress<T: Component, U: Include<T>> {
    map: InstanceMap<U>,
    phantom: PhantomData<T>
}

impl <T: Component, U: Include<T>> PropAddress<T, U> {
    pub fn new(map: InstanceMap<U>) -> PropAddress<T, U>{
        PropAddress {map, phantom: PhantomData::default()}
    }
}

// Object is Sized
pub trait Object: SizedAsset {
    // used to register an object
    fn register(asset: Self);

    // to get all instances of an object use Address()
   #[allow(non_snake_case)]
   // Self:Sized means you can't get the address of a dyn trait object
   fn Address() -> InstanceMap<Self>
       where Self:Sized;
}

pub trait Manager: SizedAsset {
    #[allow(non_snake_case)]
    fn Address() -> Rc<RefCell<Self>>;
}

pub trait UnsizedObject: Asset {}

impl <T: Object> UnsizedObject for T {}


// implement fn Metadata() for all assets that implement Default
impl <T: Asset + Default> SizedAsset for T {
    #[allow(non_snake_case)]
    fn Metadata() -> TypeMetadata {
        Self::default().type_metadata()
    }
}

#[derive(Clone)]
pub struct TypeMetadata {
    pub id: TypeId, 
    pub module_path: &'static str,
    pub type_name: &'static str
}

impl TypeMetadata {
    pub fn empty() -> Self {
        TypeMetadata{id: TypeId::empty(), module_path: "", type_name: ""}
    }
}

#[derive(Default, Clone)]
pub struct InstanceMetadata {
    pub id: Id
}

impl InstanceMetadata {
    pub fn empty() -> Self {
        InstanceMetadata{id: Id::empty()}
    }
}

// map of instances, used for components and objects
pub struct InstanceMap<T>  {
    pub map: Rc<RefCell<HashMap<Id, Rc<RefCell<T>>>>>,
    // id is the type id
    pub id: TypeId
}

impl <T> InstanceMap<T> {
    pub fn new(type_id: TypeId) -> InstanceMap<T> {
        InstanceMap{map: Rc::new(RefCell::new(HashMap::new())), id: type_id}
    }
    
}

impl <T> Clone for InstanceMap<T> {
    fn clone(&self) -> Self {
        InstanceMap{map: Rc::clone(&self.map), id: self.id}
    }
}




// Registers are any assets or asset types that can be registered
pub trait Register {
    // get the id used to register
    fn register_id(&self) -> TypeId;
}

// Implement Address for an asset instance (Rc<RefCell<U>>)
impl <T: Event, U: Receiver<T> + Asset> Address<T> for Rc<RefCell<U>> {
    // takes ownership of the address
    fn receivers(&self) -> Vec<Rc<RefCell<dyn Receiver<T> >>> {
        vec![Rc::clone(&self) as Rc<RefCell<dyn Receiver<T>>>]
    }
}

// implement Address for an instance map
impl <T: Event, U: Receiver<T> + Object> Address<T> 
    for InstanceMap<U> {
        fn receivers(&self) -> Vec<Rc<RefCell<dyn Receiver<T>>>> {
            // as keyword to cast between SmartPointer<A> to SmartPointer<dyn B>
            self.map.borrow().iter().map(|(_, instance)| {
                Rc::clone(instance) as Rc<RefCell<dyn Receiver<T>>>
                
            }).collect()
        }
}

impl <T> Register for InstanceMap<T> {
    fn register_id(&self) -> TypeId {
        self.id
    }
}

impl <T: Manager> Register for Rc<RefCell<T>> {
    fn register_id(&self) -> TypeId {
        self.borrow().type_metadata().id
    }
}

