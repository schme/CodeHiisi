use std:: {
    ops::{Deref, DerefMut},
};

use ecs::{
    world::{Fetcher, FetcherMut, World, Resource},
};

pub struct Read<'a, T: 'a>
where
    T: Resource
{
    pub val: Fetcher<'a, T>,
}

pub struct Write<'a, T: 'a>
where
    T: Resource
{
    pub val: FetcherMut<'a, T>,
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

impl<'a, T> Deref for Write<'a, T>
where
    T: Resource
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.val
    }
}

impl<'a, T> DerefMut for Write<'a, T>
where
    T: Resource
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.val
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


/// Straight from shred-cargo!
macro_rules! impl_data {
    ( $($ty:ident),* ) => {
        impl<'a, $($ty),*> SystemData<'a> for ( $( $ty , )* )
            where $( $ty : SystemData<'a> ),*
            {
                fn setup(world: &mut World) {
                    #![allow(unused_variables)]

                    $(
                        <$ty as SystemData>::setup(&mut *world);
                     )*
                }

                fn fetch(world: &'a World) -> Self {
                    #![allow(unused_variables)]

                    ( $( <$ty as SystemData<'a>>::fetch(world), )* )
                }

                //fn reads() -> Vec<ResourceId> {
                    //#![allow(unused_mut)]

                    //let mut r = Vec::new();

                    //$( {
                        //let mut reads = <$ty as SystemData>::reads();
                        //r.append(&mut reads);
                    //} )*

                    //r
                //}

                //fn writes() -> Vec<ResourceId> {
                    //#![allow(unused_mut)]

                    //let mut r = Vec::new();

                    //$( {
                        //let mut writes = <$ty as SystemData>::writes();
                        //r.append(&mut writes);
                    //} )*

                    //r
                //}
            }
    };
}

mod impl_system_data_tuples {
    use super::*;

    impl_data!(A);
    impl_data!(A, B);
    impl_data!(A, B, C);
    impl_data!(A, B, C, D);
    impl_data!(A, B, C, D, E);
    impl_data!(A, B, C, D, E, F);
    impl_data!(A, B, C, D, E, F, G);
    impl_data!(A, B, C, D, E, F, G, H);
}
