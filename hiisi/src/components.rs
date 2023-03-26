use crate::{
    ecs::{Component, VecStorage},
    math::{Vec2, vec2},
};

#[derive(Debug)]
#[derive(Component)]
#[storage(VecStorage)]
pub struct Position(pub Vec2);

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position {
            0: vec2(x, y),
        }
    }
}

#[derive(Debug)]
#[derive(Component)]
#[storage(VecStorage)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Velocity {
            0: vec2(x, y),
        }
    }
}

#[derive(Debug)]
#[derive(Component)]
#[storage(VecStorage)]
pub struct Size(pub Vec2);

impl Size {
    pub fn new(x: f32, y: f32) -> Self {
        Size {
            0: vec2(x, y),
        }
    }
}
