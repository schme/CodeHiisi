use crate::{
    ecs::{Component, VecStorage},
    math::Vector3,
};

#[derive(Debug)]
pub struct Color(pub Vector3<f32>);
impl Component for Color {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Default)]
pub struct Texture(pub String);
impl Component for Texture {
    type Storage = VecStorage<Self>;
}
