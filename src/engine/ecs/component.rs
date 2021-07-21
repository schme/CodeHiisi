use {
    ecs::storage::{UncheckedStorage},
};

pub trait Component: Sized {
    type Storage: UncheckedStorage<Self>;
}
