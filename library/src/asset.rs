use id::Id;

pub trait Asset {
    fn id(&self) -> Id;
    fn type_id(&self) -> Id;
}

pub trait Object: Asset {
    fn _get_object(id: Id) {
        
    }
}

pub trait Component: Asset {}

pub trait Include<T: Component> {}

pub trait Event {}

pub trait Receiver<T: Object, U: Event> {}

