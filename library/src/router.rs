// home of the main router
use crate::structs::event::Event;

static ROUTER: Mutex<Router> = Mutex::new(Router::new());

use std::{sync::{Mutex, MutexGuard}};


pub struct Router {
    events: Vec<Box<dyn Event>>,
}

impl Router {
    pub const fn new() -> Router {
        Router{events: vec![]}
    }

    pub fn instance() -> MutexGuard<'static, Router> {
        ROUTER.lock().unwrap()
    }

    pub fn register<T>(&mut self, event: T) 
        where T: Event {
        self.events.push(Box::new(event));
    }

    pub fn names(&self) -> Vec<String>{
        self.events.iter()
            .map(|e| e.name().to_owned())
            .collect()
    }
}