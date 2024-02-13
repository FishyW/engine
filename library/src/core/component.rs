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
         std::cell::RefCell<InstanceMap<dyn Include<Transform>>> = 
            std::cell::RefCell::new(InstanceMap::new(*__TRANSFORM_TYPE_ID));
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
    fn register(object: impl Include<Self> + 'static) {
        __TRANSFORM_INCLUDE_MAP.with(|map| {
            let map = map.borrow_mut();
            let mut map = map.map.borrow_mut();
            let id = object.metadata().id;
            let player = std::rc::Rc::new(std::cell::RefCell::new(object));
            let player = player as std::rc::Rc<std::cell::RefCell<dyn Include<Transform>>>;
            map.insert(id, std::rc::Rc::clone(&player));
            player
        });
    }

    #[allow(non_snake_case)]
    fn Address() -> ComponentMap<dyn Include<Self>> {
        let (map, id) = __TRANSFORM_INCLUDE_MAP.with(|map| {
            let map = map.borrow();
            (std::rc::Rc::clone(&map.map), map.id)
        });

        InstanceMap {map, id};
        todo!()
    }
  
}
