// transform (virtual) component


use crate::prelude::*;

#[derive(Default)]
pub struct Transform {
    metadata: InstanceMetadata,
    parent: TransformParent,
    pub x: i32,
    pub y: i32,
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

    fn parent(&self) -> &dyn Asset {
        &self.parent
    }
}


impl Transform {
    // object's transform will be replaced by this during registration
    pub fn new<T: TransformDependency + 'static>(parent: std::rc::Rc<std::cell::RefCell<T>>,
        metadata: InstanceMetadata, type_metadata: TypeMetadata) -> Self {
        

        Transform {
            parent: TransformParent {
                metadata, type_metadata,
                object: Some(parent)
            },
            ..Default::default()}
    }
}


pub trait TransformDependency: // dependencies are placed here
{}

impl <T: Object > TransformDependency 
    for T {}


struct TransformParent {
    object: Option<std::rc::Rc<std::cell::RefCell<dyn TransformDependency>>>,
    pub metadata: InstanceMetadata,
    pub type_metadata: TypeMetadata
}

impl Asset for TransformParent {
    fn metadata(&self) -> InstanceMetadata {
        self.metadata.clone()
    }

    fn type_metadata(&self) -> TypeMetadata {
        self.type_metadata.clone()
    }
}


impl std::ops::Deref for TransformParent {
    type Target = std::rc::Rc<std::cell::RefCell<dyn TransformDependency>>;
    fn deref(&self) -> &Self::Target {
        // as_ref() converts &Option<T> to Option<&T>
        self.object.as_ref().expect("Parent not initialized properly!")
    }
}

impl Default for TransformParent {
    fn default() -> Self {
        TransformParent { 
            object: None,
            metadata: InstanceMetadata{id: Id::empty()},
            type_metadata: TypeMetadata {id: TypeId::empty(), module_path: ""}
        }
    }
}

// allows turbofish notation
// this code is added in if there is more than 1 component dependency
// impl GetComponent<dyn TransformDependency> for std::cell::RefMut<'_, dyn TransformDependency> {
//     fn get<T: Component>(&mut self) -> &mut T 
//         where (dyn TransformDependency): IncludeUnsized<T>{
//         <dyn TransformDependency as IncludeUnsized<T>>
//             ::get(std::ops::DerefMut::deref_mut(self))
//     }
// }

// receiver code
use crate::ClickEvent;
impl Receiver<ClickEvent> for Transform {
    fn receive(&mut self, event: Incoming<ClickEvent>) {
        log::debug!("Transform received!");
        self.propagate(event);
    }
}

#[wasm_bindgen]
pub fn __init_receiver_hashxxx2() {
    ClickEvent::register(Transform::Address());
}
