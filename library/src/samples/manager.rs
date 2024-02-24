
use crate::{core::event::ClickEvent, prelude::*};

// #[asset(manager)]
#[derive(Default)]
pub struct SampleManager {
    metadata: InstanceMetadata
}

lazy_static::lazy_static!(
    static ref __GAME_MANAGER_TYPE_ID: TypeId = TypeId::default();
);


impl Asset for SampleManager {
    fn metadata(&self) -> InstanceMetadata {
        InstanceMetadata {..self.metadata}
    }

    #[allow(non_snake_case)]
    fn type_metadata(&self) -> TypeMetadata {
        TypeMetadata {id: *__GAME_MANAGER_TYPE_ID, 
            module_path: module_path!(),
            type_name: "SampleManager"}
    }
}


thread_local! {
    static __GAME_MANAGER_MANAGER_INSTANCE: 
        std::rc::Rc<std::cell::RefCell<SampleManager>> = 
        Default::default();
}

impl Manager for SampleManager {
    #[allow(non_snake_case)]
    fn Address() -> std::rc::Rc<std::cell::RefCell<Self>> {
        __GAME_MANAGER_MANAGER_INSTANCE.with(|instance| {
            std::rc::Rc::clone(&instance)
        })
    }
}

impl Receiver<ClickEvent> for SampleManager {
    fn receive(&mut self, event: Incoming<ClickEvent>) {
        log::debug!("Manager received!");
    }
}   

// #[wasm_bindgen]
// pub fn __init_receiver_hashxxx12222() {
//     ClickEvent::register(SampleManager::Address());
// }
