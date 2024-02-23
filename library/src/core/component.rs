// transform (virtual) component

use crate::prelude::*;

#[derive(Default)]
pub struct Transform {
    metadata: InstanceMetadata,
    pub metadata_component: ComponentMetadata,
    pub x: i32,
    pub y: i32
}

lazy_static::lazy_static!(
    static ref __TRANSFORM_TYPE_ID: TypeId = TypeId::default();
);


thread_local!{
    static __TRANSFORM_INCLUDE_MAP: ComponentMap<Transform> = 
            ComponentMap::new(*__TRANSFORM_TYPE_ID);
}

impl Asset for Transform {
    fn metadata(&self) -> InstanceMetadata {
        InstanceMetadata {..self.metadata}
    }

    fn type_metadata(&self) -> TypeMetadata {
        TypeMetadata {id: *__TRANSFORM_TYPE_ID, 
            module_path: module_path!()}
    }
}


impl Component for Transform {

    fn propagate<T: Event>(&self, event: T) {
        event.propagate(self);
    }

    fn component_metadata(&self) -> ComponentMetadata {
        self.metadata_component
    }

    fn register<T: Include<Transform> + 'static>(object_id: TypeId, object: InstanceMap<T>) {
        __TRANSFORM_INCLUDE_MAP.with(|map| {
            let mut map = map.map.borrow_mut();
            map.insert(object_id, Box::new(object));
        });
    }

    #[allow(non_snake_case)]
    fn Address() -> ComponentMap<Self> {
        let (map, id) = __TRANSFORM_INCLUDE_MAP.with(|map| {
            (std::rc::Rc::clone(&map.map), map.id)
        });

        ComponentMap {map, id}
    }
}

use crate::ClickEvent;
impl Receiver<ClickEvent> for Transform {
    fn receive(&mut self, event: ClickEvent) {
        log::debug!("Transform received!");
        self.propagate(event);
    }
}


#[wasm_bindgen]
pub fn __init_receiver_hashxxx2() {
    ClickEvent::register(Transform::Address());
}
