use {
    engine::{
        ecs::{
            Component, NullStorage,
        },
    },
};

pub use {
    renderer::components::*,
    engine::components::*,
};



#[derive(Debug, Default)]
pub struct FollowingMouse;
impl Component for FollowingMouse {
    type Storage = NullStorage<Self>;
}
