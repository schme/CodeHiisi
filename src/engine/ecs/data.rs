use std:: {
    cell::{Ref, RefMut},
    ops::{Deref, DerefMut},
    marker::{PhantomData},
};

use ecs::{
    world::{World, Resource},
};

pub struct Fetcher<'a, T: 'a> {
    pub val: Ref<'a, dyn Resource>,
    pub marker: PhantomData<&'a T>,
}

pub struct FetcherMut<'a, T: 'a> 
{
    pub val: RefMut<'a, dyn Resource>,
    pub marker: PhantomData<&'a mut T>,
}

impl<'a, T> Deref for Fetcher<'a, T>
where
    T: Resource
{
    type Target = T;

    fn deref(&self) -> &T {
        (*self.val).as_any().downcast_ref()
            .expect("Could not downcast Fetcher!")
    }
}

impl<'a, T> Deref for FetcherMut<'a, T>
where
    T: Resource
{
    type Target = T;

    fn deref(&self) -> &T {
        (*self.val).as_any().downcast_ref()
            .expect("Could not downcast Fetcher!")
    }
}

impl<'a, T> DerefMut for FetcherMut<'a, T>
where
    T: Resource
{
    fn deref_mut(&mut self) -> &mut T {
        (*self.val).as_any_mut().downcast_mut()
            .expect("Could not downcast Fetcher!")
    }
}

impl<'a, T> DynamicData<'a> for T
where
    T: SystemData<'a>,
{
    fn setup(&mut self, world: &mut World) {
        T::setup(world);
    }

    fn fetch(&self, world: &'a World) -> Self {
        T::fetch(world)
    }
}

pub trait DynamicData<'a> {

    /// Register yourself in the world
    fn setup(&mut self, world: &mut World);

    /// Get yourself from the world
    fn fetch(&self, world: &'a World) -> Self;
}

pub trait SystemData<'a> {
    fn setup(world: &mut World);
    fn fetch(world: &'a World) -> Self;
}
