
use crate::engine::prelude::*;


#[derive(Default)]
pub struct Player {
    metadata: InstanceMetadata
}



thread_local!{
    static __PLAYER_INSTANCES_MAP: 
         std::cell::RefCell<
         std::rc::Rc<
         std::cell::RefCell<
         ahash::HashMap<Id, std::rc::Rc<
            std::cell::RefCell<Player>>>
         >>> = 
        std::cell::RefCell::new(
            std::rc::Rc::new(
                std::cell::RefCell::new(
                    ahash::HashMap::new()
                )
            )
        );
}

lazy_static::lazy_static!(
    static ref __PLAYER_TYPE_ID: Id = Id::default();
);


impl Asset for Player {
    fn metadata(&self) -> InstanceMetadata {
        InstanceMetadata {..self.metadata}
    }
   
    
    #[allow(non_snake_case)]
    fn Metadata() -> TypeMetadata {
        TypeMetadata {id: *__PLAYER_TYPE_ID, 
            module_path: module_path!()}
    }

    #[allow(non_snake_case)]
    fn Address() -> TypeAddress<Self>
            where Self:Sized {
        let map = __PLAYER_INSTANCES_MAP.with(|map| {
            std::rc::Rc::clone(&map.borrow())
        });


        TypeAddress {instances: map}
    }


    fn register(asset: Self) -> std::rc::Rc<std::cell::RefCell<Self>> {
        __PLAYER_INSTANCES_MAP.with(|map| {
            let map = map.borrow_mut();
            let mut map = map.borrow_mut();
            let id = asset.metadata().id;
            let player = std::rc::Rc::new(std::cell::RefCell::new(asset));
            map.insert(id, std::rc::Rc::clone(&player));
            player
        })
    }

    fn clear(&self) {
        __PLAYER_INSTANCES_MAP.with(|map| {
            let map = map.borrow_mut();
            let mut map = map.borrow_mut();
            map.clear();
        })
    }
}

impl Object for Player {}


use lib::core::event::ClickEvent;
impl Receiver<ClickEvent> for Player {
    fn receive(&mut self, event: ClickEvent) {
        log::debug!("Clicked!");
    }
}

#[wasm_bindgen]
pub fn init_91dh9h3h329() {    
    ClickEvent::register(Player::Address());    
}