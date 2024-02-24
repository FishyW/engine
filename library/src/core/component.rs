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
            module_path: module_path!(),
            type_name: "Transform"}
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


pub trait TransformDependency: IncludeUnsized<Transform> // dependencies are placed here
{}

impl <T: Object + IncludeUnsized<Transform> > TransformDependency 
    for T {}


// allows turbofish syntax
impl GetComponent<dyn TransformDependency> for dyn TransformDependency {
    fn get_mut<T: Component>(&mut self) -> &mut T
        where dyn TransformDependency: IncludeUnsized<T> {
        <dyn TransformDependency as IncludeUnsized<T>>
            ::retrieve_mut(self)
    }

    fn get<T: Component>(&self) -> &T
            where dyn TransformDependency: IncludeUnsized<T> {
        <dyn TransformDependency as IncludeUnsized<T>>
            ::retrieve(self)
    }
}

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
            metadata: InstanceMetadata::empty(),
            type_metadata: TypeMetadata::empty()
        }
    }
}



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
