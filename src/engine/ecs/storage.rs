use {
    ecs::entity::{Index, Entity},
    ecs::component::{Component},
};

// Unsafe yet faster access
pub trait UncheckedStorage<T> {

    unsafe fn get(&self, id: Index) -> &T;
    //unsafe fn get_mut(&mut self, id: Index) -> Self::AccessMut<'_>;
    unsafe fn insert(&mut self, id: Index, value: T);
}

#[derive(Debug)]
pub struct SimpleStorage<T>(Vec<T>);

impl<T> UncheckedStorage<T> for SimpleStorage<T> 
where
    T: Default,
{
    unsafe fn get(&self, id: Index) -> &T {
        self.0.get_unchecked(id as usize)
    }

    unsafe fn insert(&mut self, id: Index, value: T) {
        let id = id as usize;
        if self.0.len() <= id {
            self.0.resize_with(id, Default::default);
            self.0.push(value)
        } else {
            self.0[id] = value;
        }
    }
}

impl<T> SimpleStorage<T> {
    pub fn get_raw_storage(&self) -> &Vec<T> {
        &self.0
    }
    pub fn get_raw_storage_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
    pub fn replace_storage(&mut self, new_vec: Vec<T>) {
        self.0 = new_vec;
    }
}
