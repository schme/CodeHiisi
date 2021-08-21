use crate::{
    ecs::{Component, VecStorage},
    math::{Vec2, vec2},
};

#[derive(Debug)]
pub struct Position(pub Vec2);
impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position {
            0: vec2(x, y),
        }
    }
}

#[derive(Debug)]
pub struct Velocity(pub Vec2);
impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity {
            0: vec2(x, y),
        }
    }
}

#[derive(Debug)]
pub struct Size(pub Vec2);
impl Component for Size {
    type Storage = VecStorage<Self>;
}

impl Size {
    pub fn new(x: f32, y: f32) -> Self {
        Size {
            0: vec2(x, y),
        }
    }
}
