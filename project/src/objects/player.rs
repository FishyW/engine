use crate::engine::prelude::*;


// object generated code
#[derive(DefaultObject)]
pub struct Player {
    metadata: InstanceMetadata,
    #[component]
    transform: Transform
}

lazy_static::lazy_static!(
    static ref __PLAYER_TYPE_ID: TypeId = TypeId::default();
);

thread_local!{

    static __PLAYER_INSTANCES_MAP: InstanceMap<Player> = 
         InstanceMap::new(*__PLAYER_TYPE_ID)
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
    fn register(object: Self) -> std::rc::Rc<std::cell::RefCell<Self>> {

        __PLAYER_INSTANCES_MAP.with(|map| {
            let mut map = map.map.borrow_mut();
            let id = object.metadata().id;
            let player = std::rc::Rc::new(std::cell::RefCell::new(object));
            map.insert(id, std::rc::Rc::clone(&player));
            player
        })
    }

    #[allow(non_snake_case)]
    fn Address() -> InstanceMap<Self> {
         __PLAYER_INSTANCES_MAP.with(|map| {
            // internally this calls Rc::clone()
            map.clone()
        })
    }
}

use lib::core::component::Transform;


// receiver generated code
use lib::core::event::ClickEvent;
// impl Receiver<ClickEvent> for Player {
//     fn receive(&mut self, _: ClickEvent) {
//         // log::debug!("Player Received");
//     }
// }

impl PropReceiver<ClickEvent, Transform> for Player {
    fn receive(&mut self, event: ClickEvent) {
        log::debug!("Prop Received!");
    }
}

#[wasm_bindgen]
pub fn __init_receiver_hashxxx44() {
    ClickEvent::prop_register(<Player as Include<Transform>>::PropAddress());
}

#[wasm_bindgen]
pub fn __init_receiver_hashxxx() {
    // ClickEvent::register(Player::Address());
}

// Include Generated Code, along with the transform: attribute
impl IncludeUnsized<Transform> for Player {
    fn get<'a>(&'a mut self) -> &'a mut Transform {
        &mut self.transform
    }
}

impl Include<Transform> for Player {
   fn PropAddress() -> PropAddress<Transform, Self> {
       __PLAYER_INSTANCES_MAP.with(|map| {
            PropAddress::new(map.clone())
       })
   }
}


#[wasm_bindgen]
pub fn __init_include_hashxxx3() {
    Transform::register::<Player>(*__PLAYER_TYPE_ID, Object::Address());
}

// #[wasm_bindgen]
// pub fn __init_test() {
//     let player = Player::default();
    
//     log::debug!("Player: \n {:?}\n {:?}", player.metadata.id, player.type_metadata().id);
//     log::debug!("Transform: \n {:?} \n {:?}", player.transform.metadata_component.parent_id, 
//     player.transform.metadata_component.parent_typeid);
// }


