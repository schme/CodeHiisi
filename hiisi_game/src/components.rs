use {
    hiisi::ecs::{
        Component, NullStorage,
    },
};

#[derive(Debug, Default)]
pub struct FollowingMouse;
impl Component for FollowingMouse {
    type Storage = NullStorage<Self>;
}
