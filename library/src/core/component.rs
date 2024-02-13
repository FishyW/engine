// transform (virtual) component


use crate::prelude::*;

#[derive(Default)]
pub struct Transform {
    metadata: InstanceMetadata,
    pub x: i32,
    pub y: i32
}


lazy_static::lazy_static!(
    static ref __TRANSFORM_TYPE_ID: Id = Id::default();
);


thread_local!{
    static __TRANSFORM_INCLUDE_MAP: 
         std::cell::RefCell<ComponentMap<Transform>> = 
            std::cell::RefCell::new(ComponentMap::new(*__TRANSFORM_TYPE_ID));
}



impl Asset for Transform {
    fn metadata(&self) -> InstanceMetadata {
        InstanceMetadata {..self.metadata}
    }

    fn type_metadata(&self) -> TypeMetadata {
        TypeMetadata {id: *__TRANSFORM_TYPE_ID, 
            module_path: module_path!()}
    }

    fn clean(self) {
        __TRANSFORM_INCLUDE_MAP.with(|map| {
            let map = map.borrow_mut();
            let mut map = map.map.borrow_mut();
            map.clear();
        })
    }
}


impl Component for Transform {
    fn register<T: Include<Transform> + 'static>(component_id: Id, object: InstanceMap<T>) {
        __TRANSFORM_INCLUDE_MAP.with(|map| {
            let map = map.borrow_mut();
            let mut map = map.map.borrow_mut();
            map.insert(component_id, Box::new(object));
        });
    }

    #[allow(non_snake_case)]
    fn Address() -> ComponentMap<Self> {
        let (map, id) = __TRANSFORM_INCLUDE_MAP.with(|map| {
            let map = map.borrow();
            (std::rc::Rc::clone(&map.map), map.id)
        });

        ComponentMap {map, id}
    }
  
}
