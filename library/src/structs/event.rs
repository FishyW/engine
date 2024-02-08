use crate::router::Router;

trait EventPrivate {
    fn init(self)
    where
        Self: Sized + Event,
    {
        Router::instance().register(self);
    }
}

// supertrait
pub trait Event: Send + Sync + 'static {
    fn new()
    where
        Self: Sized;
    fn name(&self) -> &'static str;
}

impl<T: Event> EventPrivate for T {}

pub struct RandomEvent {
    name: &'static str,
}

impl Event for RandomEvent {
    fn new() {
        (RandomEvent { name: "hello" }).init();
    }

    fn name(&self) -> &'static str {
        self.name
    }
}
