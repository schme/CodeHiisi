use std::sync::atomic::{AtomicUsize, Ordering};

use super::component::{OldComponent};

static ENTITY_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct Entity {
    pub id: usize,
    pub components: Vec<OldComponent>,
}

impl Entity {
    pub fn new() -> Entity {
        let id = ENTITY_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        Entity {id, components: Vec::new()}
    }

    pub fn add_component(&mut self, component: OldComponent) {
        self.components.push(component);
    }
}
