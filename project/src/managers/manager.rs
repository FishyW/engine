use lib::core::event::ClickEvent;

use crate::engine::prelude::*;

#[derive(Default)]
struct GameManager {
    metadata: InstanceMetadata
}

lazy_static::lazy_static!(
    static ref __GAME_MANAGER_TYPE_ID: TypeId = TypeId::default();
);


impl Asset for GameManager {
    fn metadata(&self) -> InstanceMetadata {
        InstanceMetadata {..self.metadata}
    }

    #[allow(non_snake_case)]
    fn type_metadata(&self) -> TypeMetadata {
        TypeMetadata {id: *__GAME_MANAGER_TYPE_ID, 
            module_path: module_path!()}
    }
}


thread_local! {
    static __GAME_MANAGER_MANAGER_INSTANCE: 
        std::rc::Rc<std::cell::RefCell<GameManager>> = 
        Default::default();
}

impl Manager for GameManager {
    #[allow(non_snake_case)]
    fn Address() -> std::rc::Rc<std::cell::RefCell<Self>> {
        __GAME_MANAGER_MANAGER_INSTANCE.with(|instance| {
            std::rc::Rc::clone(&instance)
        })
    }
}

impl Receiver<ClickEvent> for GameManager {
    fn receive(&mut self, event: ClickEvent) {
        log::debug!("Manager received!");
    }
}   

#[wasm_bindgen]
pub fn __init_receiver_hashxxx332() {
    ClickEvent::register(GameManager::Address());
}
