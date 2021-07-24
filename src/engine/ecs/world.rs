///
/// Initial version totally plagiarized, partly paraphrazed etc. from
/// the shred library and similar rust ECS frameworks
///
use std::{
    any::{Any, TypeId},
    cell::{RefCell},
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
    marker::{PhantomData},
    ops::{Deref, DerefMut},
};

use ecs::{
    entity::{Entities, Entity},
    data::{Fetcher, FetcherMut, DynamicData},
};


pub struct Read<'a, T: 'a>
where
    T: Resource
{
    val: Fetcher<'a, T>,
}

pub struct Write<'a, T: 'a>
where
    T: Resource
{
    val: FetcherMut<'a, T>,
}

impl<'a, T> Deref for Read<'a, T>
where
    T: Resource
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.val
    }
}

impl<'a, T> DynamicData<'a> for Read<'a, T> 
where
    T: Resource
{
    fn setup(&mut self, world: &mut World) {

    }

    fn fetch(&self, world: &'a World) -> Self {
        world.get::<T>().into()
    }
}

impl<'a, T> From<Fetcher<'a, T>> for Read<'a, T>
where
    T: Resource
{
    fn from(val: Fetcher<'a, T>) -> Self {
        Read {
            val,
        }
    }
}

//impl<'a, T> Deref for Write<'a, T>
//where
    //T: Resource
//{
    //type Target = T;

    //fn deref(&self) -> &Self::Target {
        //&*self.val.borrow()
    //}
//}

//impl<'a, T> DerefMut for Write<'a, T>
//where
    //T: Resource
//{
    //type Target = T;

    //fn deref_mut(&mut self) -> &mut Self::Target {
        //&*mut self.val
    //}
//}


impl<'a, T> Read<'a, T>
where
    T: Resource
{

}

impl<'a, T> Write<'a, T> 
where
    T: Resource
{

}

pub trait Resource: Any + 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T> Resource for T where T: Any {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

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
    // TODO: make thread safe, not sure how should be implemented right now
    resources: HashMap<ResourceId, RefCell<Box<dyn Resource>>>,
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
    pub fn get<T>(&self) -> Fetcher<T>
    where
        T: Resource
    {
        let val = &**self.resources.get(&ResourceId::new::<T>()).unwrap().borrow();
        Fetcher {
            val,
            marker: PhantomData,
        }
    }

    pub fn get_mut<T>(&mut self) -> FetcherMut<T>
    where
        T: Resource
    {
        let val = self.resources.get_mut(&ResourceId::new::<T>()).unwrap().borrow_mut();
        FetcherMut {
            val,
            marker: PhantomData,
        }
    }

    pub fn insert_default<T>(&mut self)
    where
        T: Resource + Default
    {
        self.insert::<T>(Default::default());
    }

    ///
    /// Local only
    /// 
    fn insert_by_id<T>(&mut self, id: ResourceId, res: T)
    where
        T: Resource
    {
        self.resources.insert(id, RefCell::new(Box::new(res)));
    }

}
