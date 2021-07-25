///
/// Initial version totally plagiarized, partly paraphrazed etc. from
/// the shred library and similar rust ECS frameworks
///
use std::{
    any::{Any, TypeId},
    cell::{RefCell, Ref, RefMut},
    collections::HashMap,
    marker::{PhantomData},
    ops::{Deref, DerefMut},
};

use ecs::{
    entity::{Entities, Entity},
    data::{Fetcher, FetcherMut, SystemData},
};

macro_rules! get_panic {
    () => {{
        panic!(
            "\
            Tried to get resource with id `{:?}`[^1] from the `World`, but \
            the resource does not exist.\n\n\
            [^1]: Full type name: `{}`",
            resource_id = TypeId::of::<T>(),
            resource_name = std::any::type_name::<T>(),
        )
    }};
}

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

impl<'a, T> SystemData<'a> for Read<'a, T>
where
    T: Resource + Default
{
    fn setup(world: &mut World) {
        world.enter_resource::<T>();
    }
    fn fetch(world: &'a World) -> Self {
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

impl<'a, T> From<FetcherMut<'a, T>> for Write<'a, T>
where
    T: Resource
{
    fn from(val: FetcherMut<'a, T>) -> Self {
        Write {
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

impl<T> Resource for T
where
    T: Any
{
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

//impl<T> AsAny for T
//where
    //T: Resource
//{
    //fn as_any(&self) -> &dyn Any {
        //self
    //}
    //fn as_any_mut(&mut self) -> &mut dyn Any {
        //self
    //}
//}

//pub trait AsAny: {
    //fn as_any(&self) -> &dyn Any;
    //fn as_any_mut(&mut self) -> &mut dyn Any;
//}

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

    pub fn get<T>(&self) -> Fetcher<T>
    where
        T: Resource
    {
        self.try_get().unwrap_or_else(|| {
            if self.resources.is_empty() {
                eprintln!("Could not fetch resource, World is empty!");
            }
            get_panic!()
        })
    }

    pub fn try_get<T>(&self) -> Option<Fetcher<T>>
    where
        T: Resource
    {
        let res_id = ResourceId::new::<T>();

        self.resources.get(&res_id).map(|rc| Fetcher {
            val: Ref::map(rc.borrow(), Box::as_ref),
            marker: PhantomData,
        })
    }

    pub fn get_mut<T>(&self) -> FetcherMut<T>
    where
        T: Resource
    {
        self.try_get_mut().unwrap_or_else(|| {
            if self.resources.is_empty() {
                eprintln!("Could not fetch resource, World is empty!");
            }
            get_panic!()
        })
    }

    pub fn try_get_mut<T>(&self) -> Option<FetcherMut<T>>
    where
        T: Resource
    {
        let res_id = ResourceId::new::<T>();
        self.resources.get(&res_id).map(|rc| FetcherMut {
            val: RefMut::map(rc.borrow_mut(), Box::as_mut),
            marker: PhantomData,
        })
    }

    pub fn enter_resource_with<T>(&mut self, with: T)
    where
        T: Resource
    {
        let id = ResourceId::new::<T>();
        if !self.resources.contains_key(&id) {
            self.insert_by_id::<T>(id, with);
        }
    }

    pub fn enter_resource<T>(&mut self)
    where
        T: Resource + Default
    {
        let id = ResourceId::new::<T>();
        if !self.resources.contains_key(&id) {
            self.insert_default::<T>();
        }
    }

    pub fn insert_default<T>(&mut self)
    where
        T: Resource + Default
    {
        self.insert::<T>(Default::default());
    }

    pub fn setup<'a, T: SystemData<'a>>(&mut self) {
        T::setup(self);
    }

    pub fn system_data<'a, T>(&'a self) -> T
    where
        T: SystemData<'a>,
    {
        SystemData::fetch(&self)
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
