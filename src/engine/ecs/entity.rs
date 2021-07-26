use {
    std::{
        collections::{HashMap},
        sync::atomic::{AtomicU32, Ordering},
    },
};

static ENTITY_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

pub type Index = u32;
pub type Generation = u32;

#[derive(Debug, PartialEq)]
pub struct Entity(Index, Generation);

impl Entity {
    pub fn new() -> Self {
        let id = ENTITY_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        Entity(id, 0)
    }

    pub fn id(&self) -> Index {
        self.0
    }

    pub fn gen(&self) -> Generation {
        self.1
    }
}

#[derive(Debug)]
pub struct Entities {
    index_map: HashMap<Index, usize>,
    data: Vec<Entity>,
}

// TODO: Remove entities
// TODO: Reuse unused indices
impl Entities {

    pub fn new() -> Self {
        Entities{ index_map: HashMap::new(), data: Vec::with_capacity(8)}
    }

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

    pub fn len(&self) -> usize {
        self.data.len()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_entities() {
        let mut entities = Entities::new();

        for _ in 0..10 {
            entities.new_entity();
        }

        assert_eq!(entities.len(), 10);
    }

    #[test]
    fn get_entities() {
        let mut entities = Entities::new();

        entities.new_entity();
        entities.new_entity();
        let id = entities.new_entity().id();
        entities.new_entity();

        assert_eq!(id, entities.get(id).id());
    }
}
