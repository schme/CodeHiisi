use engine::{
    ecs::{Component, VecStorage},
    math::{Vector2, Point2},
};

#[derive(Debug)]
pub struct Position(pub Point2<f32>);
impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Velocity(pub Vector2<f32>);
impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Size(pub Vector2<f32>);
impl Component for Size {
    type Storage = VecStorage<Self>;
}
