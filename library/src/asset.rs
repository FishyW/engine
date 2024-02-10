mod event;

use std::{cell::RefCell, rc::Rc};

pub use event::*;
pub use crate::prelude::*;

pub trait Asset: 'static + Default {
    fn metadata(&self) -> InstanceMetadata;

    fn type_metadata(&self) -> TypeMetadata {
        Self::Metadata()
    }

    #[allow(non_snake_case)]
    fn Metadata() -> TypeMetadata;

    // to get all instances of an object use Address()
    #[allow(non_snake_case)]
    // Self:Sized means you can't get the address of a dyn trait object
    fn Address() -> TypeAddress<Self>
        where Self:Sized;


        fn register(asset: Self) -> std::rc::Rc<std::cell::RefCell<Self>>;

    fn clear(&self);

}

pub struct TypeMetadata {
    pub id: Id, 
    pub module_path: &'static str
}

#[derive(Default)]
pub struct InstanceMetadata {
    pub id: Id
}

pub trait Object: Asset {}

pub trait Component: Asset {}

pub trait Include<T: Component>: Object {}


