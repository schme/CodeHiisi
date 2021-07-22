///
/// Initial version totally plagiarized, partly paraphrazed etc. from
/// the shred library.
///
use std::{
    collections::HashMap,
    any::{Any, TypeId},
};

use {
    ecs::entity::{Entities, Entity},
    ecs::component::{Component},
};

pub trait Resource: Any + 'static {}

/// Copied from the shred library as is. Not sure how these should be
/// properly marked, but here's an attempt!
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ResourceId {
    type_id: TypeId,
    dynamic_id: u64,
}

impl ResourceId {

    #[inline]
    pub fn new<T: Resource>() -> Self {
        ResourceId::new_with_dynamic_id::<T>(0)
    }

    #[inline]
    pub fn from_type_id(type_id: TypeId) -> Self {
        ResourceId::from_type_id_and_dynamic_id(type_id, 0)
    }
    #[inline]
    pub fn new_with_dynamic_id<T: Resource>(dynamic_id: u64) -> Self {
        ResourceId::from_type_id_and_dynamic_id(TypeId::of::<T>(), dynamic_id)
    }

    #[inline]
    pub fn from_type_id_and_dynamic_id(type_id: TypeId, dynamic_id: u64) -> Self {
        ResourceId {
            type_id,
            dynamic_id,
        }
    }
}

pub struct World {
    resources: HashMap<ResourceId, Box<dyn Resource>>,
}

impl World {
    pub fn new() -> Self {
        World { resources: HashMap::new() }
    }

    pub fn insert<T>(&mut self, res: T)
    where
        T: Resource
    {
        self.insert_by_id(ResourceId::new::<T>(), res);
    }

    // Really need to walk through these things
    pub fn get_resource<T>(&self) -> Option<Box<dyn Resource>>
    where
        T: Resource
    {
        self.resources.get(ResourceId::from_type_id(TypeId::of::<T>()))
    }

    fn insert_by_id<T>(&mut self, id: ResourceId, res: T)
    where
        T: Resource,
    {
        self.resources.insert(id, Box::new(res));
    }

}
