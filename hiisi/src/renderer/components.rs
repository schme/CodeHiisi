use crate::{
    ecs::{Component, VecStorage},
};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Color(pub f32, pub f32, pub f32);
impl Component for Color {
    type Storage = VecStorage<Self>;
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color(r, g, b)
    }
}

#[derive(Debug, Default)]
pub struct Texture(pub String);
impl Component for Texture {
    type Storage = VecStorage<Self>;
}
