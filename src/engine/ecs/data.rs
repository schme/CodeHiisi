use std:: {
    ops::{Deref, DerefMut},
    marker::{PhantomData},
};

use ecs::{
    world::{World, Resource},
};

pub struct Fetcher<'a, T> 
where
    T: Resource,
{
    val: &'a dyn Resource,
    marker: PhantomData<T>,
}

pub struct FetcherMut<'a, T> 
where
    T: Resource,
{
    val: &'a mut dyn Resource,
    marker: PhantomData<T>,
}

impl<'a, T> Deref for Fetcher<'a, T>
where
    T: Resource
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.val.as_any().downcast_ref().unwrap() }
    }
}

//impl<'a, T> for FetcherMut<'a, T> {
    //fn fetch(&self, world: &'a World) -> Self {
        //world.get_mut::<T>()
    //}
//}


pub trait DynamicData<'a> {

    /// Register yourself in the world
    fn setup(&self, world: &mut World);

    /// Get yourself from the world
    fn fetch(&self, world: &'a World) -> Self;


    // TODO: read/write bookkeeping
}
