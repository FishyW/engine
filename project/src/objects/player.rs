
use crate::engine::prelude::*;


// object generated code
#[derive(Default)]
pub struct Player {
    metadata: InstanceMetadata,
    transform: Transform
}

lazy_static::lazy_static!(
    static ref __PLAYER_TYPE_ID: Id = Id::default();
);

thread_local!{
    static __PLAYER_INSTANCES_MAP: 
         std::cell::RefCell<InstanceMap<Player>> = 
         std::cell::RefCell::new(InstanceMap::new(*__PLAYER_TYPE_ID))
}



impl Asset for Player {
    fn metadata(&self) -> InstanceMetadata {
        InstanceMetadata {..self.metadata}
    }

    #[allow(non_snake_case)]
    fn type_metadata(&self) -> TypeMetadata {
        TypeMetadata {id: *__PLAYER_TYPE_ID, 
            module_path: module_path!()}
    }
}


impl Object for Player {
    fn register(asset: Self) -> std::rc::Rc<std::cell::RefCell<Self>> {
        __PLAYER_INSTANCES_MAP.with(|map| {
            let map = map.borrow_mut();
            let mut map = map.map.borrow_mut();
            let id = asset.metadata().id;
            let player = std::rc::Rc::new(std::cell::RefCell::new(asset));
            map.insert(id, std::rc::Rc::clone(&player));
            player
        })
    }

    #[allow(non_snake_case)]
    fn Address() -> InstanceMap<Self> {
        let (map, id) = __PLAYER_INSTANCES_MAP.with(|map| {
            let map = map.borrow();
            (std::rc::Rc::clone(&map.map), map.id)
        });

        InstanceMap {map, id}
    }
}

use lib::core::component::Transform;


// receiver generated code
use lib::core::event::ClickEvent;
impl Receiver<ClickEvent> for Player {
    fn receive(&mut self, _: ClickEvent) {
        log::debug!("Clicked!");
    }
}

impl Include<Transform> for Player {
    fn get<'a>(&'a mut self) -> &'a mut Transform {
        &mut self.transform
    }
}

// include register
#[wasm_bindgen]
pub fn __init_include_hashxxx3() {
    Transform::register::<Player>(*__PLAYER_TYPE_ID, Object::Address());
}


#[wasm_bindgen]
pub fn __init_receiver_hashxxx() {
    ClickEvent::register(Player::Address());
}

#[wasm_bindgen]
pub fn __init_ee() {
    let event = ClickEvent{};
    event.broadcast();
}
