use crate::engine::prelude::*;


#[derive(Default)]
struct Player {
    __id: Id,
}



use ahash::HashMapExt;
thread_local!{
    static __PLAYER_INSTANCES_MAP: 
         std::cell::RefCell<
         std::rc::Rc<
         ahash::HashMap<Id, std::rc::Rc<Player>>
         >> = 
        std::cell::RefCell::new(
            std::rc::Rc::new(
                ahash::HashMap::new()
            )
        );
}

lazy_static::lazy_static!(
    static ref __PLAYER_TYPE_ID: Id = Id::default();
);


impl Asset for Player {
    fn metadata(&self) -> InstanceMetadata {
        InstanceMetadata {id: self.__id}
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

    fn to_ref(&self) -> std::rc::Rc<Self>
            where Self:Sized {
        __PLAYER_INSTANCES_MAP.with(|map| {
            let map = map.borrow();
            let instance = map.get(&self.metadata().id)
                .expect("Id not found in the map!");
            std::rc::Rc::clone(&instance)
        })
    }
}

impl Object for Player {}


use lib::event::ClickEvent;
impl Receiver<ClickEvent> for Player {
    fn receive(&self, event: ClickEvent) {
        log::debug!("Received!");
    }
}