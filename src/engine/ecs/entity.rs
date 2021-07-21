use {
    std::{
        collections::{HashMap},
        sync::atomic::{AtomicU32, Ordering},
    },
};

static ENTITY_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

pub type Index = u32;
pub type Generation = u32;

#[derive(Debug)]
pub struct Entities {
    index_map: HashMap<Index, usize>,
    data: Vec<Entity>,
}

// TODO: Remove entities
// TODO: Reuse unused indices
impl Entities {
    pub fn new_entity(&mut self) -> &mut Entity {
        let entity = Entity::new();
        let entity_id = entity.0;
        self.index_map.insert(entity_id, self.data.len());
        self.data.push(entity);
        self.get_mut(entity_id)
    }

    pub fn get(&self, entity_id : Index) -> &Entity {
        &self.data[*self.index_map.get(&entity_id).unwrap()]
    }

    pub fn get_mut(&mut self, entity_id : Index) -> &mut Entity {
        &mut self.data[*self.index_map.get(&entity_id).unwrap()]
    }
}

#[derive(Debug)]
pub struct Entity(Index, Generation);

impl Entity {
    pub fn new() -> Self {
        let id = ENTITY_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        Entity(id, 0)
    }
}
